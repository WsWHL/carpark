use stm32f1xx_hal::gpio::{self, gpiob, IOPinSpeed, Output, OutputSpeed, PinState};

#[doc = "蜂鸣器设备"]
pub struct BuzzerDevice {
    buzzer: gpiob::PB0<Output>,
}

impl BuzzerDevice {
    pub fn new(pb0: gpiob::PB0, crl: &mut gpio::Cr<'B', false>) -> Self {
        let mut buzzer = pb0.into_push_pull_output_with_state(crl, PinState::High);
        buzzer.set_speed(crl, IOPinSpeed::Mhz50);

        Self { buzzer }
    }

    pub fn on(&mut self) {
        self.buzzer.set_low();
    }

    pub fn off(&mut self) {
        self.buzzer.set_high();
    }
}
