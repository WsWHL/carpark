#![allow(dead_code)]

use stm32f1::stm32f103::TIM2;
use stm32f1xx_hal::{
    afio,
    gpio::{self, gpioa, Alternate},
    prelude::*,
    rcc,
    time::ms,
    timer::{Ch, Channel, PwmHz, SysDelay, Tim2NoRemap},
};

use crate::base::pac;

#[doc = "SG90舵机设备"]
pub struct SGPwmDevice {
    pwm: PwmHz<TIM2, Tim2NoRemap, Ch<1>, gpioa::PA1<Alternate>>,
    delay: SysDelay,
}

impl SGPwmDevice {
    pub fn new(
        tim2: TIM2,
        pa1: gpioa::PA1,
        clocks: rcc::Clocks,
        crl: &mut gpio::Cr<'A', false>,
        mapr: &mut afio::MAPR,
    ) -> Self {
        let pa1 = pa1.into_alternate_push_pull(crl);

        let mut pwm =
            tim2.pwm_hz::<Tim2NoRemap, Ch<1>, gpioa::PA1<Alternate>>(pa1, mapr, 1.kHz(), &clocks);
        pwm.enable(Channel::C2);
        pwm.set_period(ms(20).into_rate());

        let cp = pac::Peripherals::ctake().unwrap();
        let delay = cp.SYST.delay(&clocks);

        Self { pwm, delay }
    }

    pub fn on(&mut self) {
        self.pwm.set_duty(Channel::C2, 6666.6 as u16);

        self.delay.delay_ms(1000_u16);

        self.pwm.set_duty(Channel::C2, 3999.9 as u16);
    }
}
