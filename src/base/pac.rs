#![allow(dead_code)]

use cortex_m;
use stm32f1xx_hal::pac;

pub struct Peripherals;

impl Peripherals {
    #[doc = "Returns all the peripherals"]
    pub fn take() -> Option<pac::Peripherals> {
        cortex_m::interrupt::free(|_| Some(unsafe { pac::Peripherals::steal() }))
    }

    #[doc = "Returns cortex all the peripherals"]
    pub fn ctake() -> Option<cortex_m::Peripherals> {
        cortex_m::interrupt::free(|_| Some(unsafe { cortex_m::Peripherals::steal() }))
    }
}
