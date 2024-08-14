use core::fmt::Write;

use stm32f1::stm32f103::USART1;
use stm32f1xx_hal::{
    afio,
    gpio::{self, IOPinSpeed, Output, OutputSpeed},
    prelude::{_embedded_hal_blocking_delay_DelayMs, _stm32_hal_time_U32Ext},
    rcc,
    serial::{Config, Rx, Serial, Tx},
    timer::{SysDelay, SysTimerExt},
};

use crate::base::{gb2312, pac};

#[doc = "LU6288语音播报设备"]
pub struct VoiceDevice {
    busy: gpio::PA8<Output>,
    tx: Tx<USART1>,
    _rx: Rx<USART1>,
    delay: SysDelay,
}

impl VoiceDevice {
    pub fn new(
        usart: USART1,
        pa8: gpio::PA8,
        pa9: gpio::PA9,
        pa10: gpio::PA10,
        clocks: rcc::Clocks,
        crh: &mut gpio::Cr<'A', true>,
        mapr: &mut afio::MAPR,
    ) -> Self {
        let busy = pa8.into_push_pull_output(crh);
        let mut tx = pa9.into_alternate_push_pull(crh);
        let rx = pa10;
        tx.set_speed(crh, IOPinSpeed::Mhz50);

        let serial = Serial::new(
            usart,
            (tx, rx),
            mapr,
            Config::default().baudrate(9600.bps()),
            &clocks,
        );

        let (tx, _rx) = serial.split();

        let cp = pac::Peripherals::ctake().unwrap();
        let delay = cp.SYST.delay(&clocks);

        Self {
            busy,
            tx,
            _rx,
            delay,
        }
    }

    pub fn broadcast(&mut self, s: &str) {
        let bytes = gb2312::str_to_gb2312(s);

        self.busy.set_high();
        if self.busy.is_set_high() {
            self.tx.write_str("<G>").ok();
            for b in bytes {
                nb::block!(self.tx.write(b)).ok();
            }
        }
        self.delay.delay_ms(1000_u32);
        self.busy.set_low();
    }
}
