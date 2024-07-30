#![allow(dead_code)]

use stm32f1xx_hal::gpio::{self, gpioa, Output};

#[doc = "LED灯设备"]
pub struct LEDDevice {
    leds: [gpio::ErasedPin<Output>; 2],
}

impl LEDDevice {
    pub fn new(pa2: gpioa::PA2, pa3: gpioa::PA3, crl: &mut gpio::Cr<'A', false>) -> Self {
        let leds = [
            pa2.into_push_pull_output_with_state(crl, gpio::PinState::High)
                .erase(),
            pa3.into_push_pull_output_with_state(crl, gpio::PinState::High)
                .erase(),
        ];

        Self { leds }
    }

    // 点亮LED灯
    pub fn on(&mut self, i: usize) {
        if i < self.leds.len() {
            self.leds[i].set_low();
        }
    }

    // 关闭LED灯
    pub fn off(&mut self, i: usize) {
        if i < self.leds.len() {
            self.leds[i].set_high();
        }
    }
}
