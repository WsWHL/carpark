#![allow(dead_code)]

use crate::{base::pac, devices::*};
use hash32::{BuildHasherDefault, FnvHasher};
use heapless::{FnvIndexMap, IndexMap};
use stm32f1xx_hal::{afio::AfioExt, flash::FlashExt, gpio::GpioExt, rcc::RccExt};

pub struct Modules {
    pub oled: oled::OLEDDevice,
    pub rfid: rfid::RFIDDevice,
    pub sg: sg::SGPwmDevice,
    pub led: led::LEDDevice,
    pub buzzer: buzzer::BuzzerDevice,
    pub dht: dht::DHTDevice,
    pub voice: voice::VoiceDevice,
    pub ds: ds::DSDevice,
}

impl Modules {
    pub fn new() -> Self {
        // init devices
        let p = pac::Peripherals::take().unwrap();

        let rcc = p.RCC.constrain();
        let mut flash = p.FLASH.constrain();
        let mut afio = p.AFIO.constrain();
        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mut gpioa = p.GPIOA.split();
        let mut gpiob = p.GPIOB.split();

        // I2c configuration
        let oled = oled::OLEDDevice::new(
            p.I2C1,
            gpiob.pb8,
            gpiob.pb9,
            clocks,
            &mut gpiob.crh,
            &mut afio.mapr,
        );

        // SPI configuration
        let rfid = rfid::RFIDDevice::new(
            p.SPI2,
            gpiob.pb12,
            gpiob.pb13,
            gpiob.pb14,
            gpiob.pb15,
            clocks,
            &mut gpiob.crh,
        );

        // Pwm configuration
        let sg = sg::SGPwmDevice::new(p.TIM2, gpioa.pa1, clocks, &mut gpioa.crl, &mut afio.mapr);

        // Led configuration
        let led = led::LEDDevice::new(gpioa.pa2, gpioa.pa3, &mut gpioa.crl);

        // Buzzer configuration
        let buzzer = buzzer::BuzzerDevice::new(gpiob.pb0, &mut gpiob.crl);

        // DHT11 configuration
        let dht = dht::DHTDevice::new(gpiob.pb1, clocks, &mut gpiob.crl);

        // Voice configuration
        let voice = voice::VoiceDevice::new(
            p.USART1,
            gpioa.pa8,
            gpioa.pa9,
            gpioa.pa10,
            clocks,
            &mut gpioa.crh,
            &mut afio.mapr,
        );

        // Clock configuration
        let ds = ds::DSDevice::new(p.I2C2, gpiob.pb10, gpiob.pb11, clocks, &mut gpiob.crh);

        Self {
            oled,
            rfid,
            sg,
            led,
            buzzer,
            dht,
            voice,
            ds,
        }
    }
}

// 刷卡错误
#[derive(Debug)]
pub enum ScanErrors {
    ParkingLimit = 1001, // 车位上限
    Unknown = 1000,      // 未知错误
}

#[doc = "停车位"]
pub struct ParkingSpace {
    uid: i64,
    sts: i64,
}

#[doc = "停车场管理 N: 初始化车位数量"]
pub struct Parking<const N: usize> {
    led: led::LEDDevice,

    spaces: [ParkingSpace; N],
    cm: IndexMap<i64, usize, BuildHasherDefault<FnvHasher>, 16>,
}

impl<const N: usize> Parking<N> {
    pub fn new(led: led::LEDDevice) -> Self {
        let spaces: [ParkingSpace; N] = core::array::from_fn(|_| ParkingSpace { uid: 0, sts: 0 });
        let cm = FnvIndexMap::<i64, usize, 16>::new();

        Self { led, spaces, cm }
    }

    // 汽车入库扫描
    pub fn scanning(&mut self, uid: i64, ts: i64) -> Result<i64, ScanErrors> {
        if self.cm.contains_key(&uid) {
            Ok(self.out_car(uid))
        } else {
            if self.is_full() {
                return Err(ScanErrors::ParkingLimit);
            }

            self.in_car(uid, ts);
            Ok(ts)
        }
    }

    // 剩余车位数量
    pub fn get_idle(&self) -> usize {
        self.spaces.len() - self.cm.len()
    }

    // 车位是否已满
    fn is_full(&self) -> bool {
        self.cm.len() >= self.spaces.len()
    }

    // 汽车入库
    fn in_car(&mut self, uid: i64, ts: i64) {
        for (i, s) in self.spaces.iter_mut().enumerate() {
            if s.uid <= 0 {
                s.uid = uid;
                s.sts = ts;
                self.cm.insert(uid, i).unwrap();

                // 开启车位指示灯
                self.led.on(i);

                break;
            }
        }
    }

    // 汽车出库
    fn out_car(&mut self, uid: i64) -> i64 {
        let sts: i64;
        {
            let i = self.cm.get(&uid).unwrap();
            let s = self.spaces.get_mut(*i).unwrap();
            sts = s.sts;
            s.uid = 0;
            s.sts = 0;

            // 关闭车位指示灯
            self.led.off(*i);
        }
        self.cm.remove(&uid);

        sts
    }
}
