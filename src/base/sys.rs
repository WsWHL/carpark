#![allow(dead_code)]

use stm32f1xx_hal::rtc;
use stm32f1xx_hal::timer::delay;
use stm32f1xx_hal::{flash::FlashExt, prelude::*, rcc::RccExt, timer::SysTimerExt};

use super::pac;

#[doc = "系统功能：延时、时间"]
pub struct Sys {}

impl Sys {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_delay(&self) -> delay::SysDelay {
        let p = pac::Peripherals::take().unwrap();
        let cp = pac::Peripherals::ctake().unwrap();

        let mut flash = p.FLASH.constrain();
        let rcc = p.RCC.constrain();
        let syst = cp.SYST;

        let clocks = rcc.cfgr.hclk(8.MHz()).freeze(&mut flash.acr);

        syst.delay(&clocks)
    }

    pub fn time_rtc(&self) -> rtc::Rtc {
        let p = pac::Peripherals::take().unwrap();

        let rcc = p.RCC.constrain();
        let mut pwr = p.PWR;

        let mut backup_domains = rcc.bkp.constrain(p.BKP, &mut pwr);
        let mut rtc = rtc::Rtc::new(p.RTC, &mut backup_domains);

        if rtc.current_time() == 0 {
            rtc.set_time(1720875552);
        }

        rtc
    }
}
