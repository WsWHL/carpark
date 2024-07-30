#![allow(dead_code)]

use crate::base::sys;
use stm32f1xx_hal::gpio::{self, gpiob, Input, PullUp};
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::timer;

#[doc = "按键设备"]
pub struct KeysDevice {
    b0: gpiob::PB0<Input<PullUp>>,
    delay: timer::delay::SysDelay,
}

impl KeysDevice {
    pub fn new(pb0: gpiob::PB0, crl: &mut gpio::Cr<'B', false>) -> Self {
        let b0 = pb0.into_pull_up_input(crl);

        let s = sys::Sys::new();
        let delay = s.get_delay();

        Self { b0, delay }
    }

    pub fn is_press(&mut self) -> bool {
        if self.b0.is_low() {
            self.delay.delay_ms(10_u16);
            while self.b0.is_low() {}
            self.delay.delay_ms(10_u16);
            true
        } else {
            false
        }
    }
}
