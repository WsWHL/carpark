#![allow(dead_code)]

use crate::base::{pac, strings::Strfmt};
use mfrc522::Mfrc522;
use stm32f1xx_hal::{
    gpio::{self, gpiob, Alternate, Floating, Input, Output, PushPull},
    pac::SPI2,
    prelude::{_embedded_hal_blocking_delay_DelayMs, _fugit_RateExtU32},
    rcc,
    spi::{Mode, Phase, Polarity, Spi, Spi2NoRemap},
    timer::{SysDelay, SysTimerExt},
};

use super::buzzer;

#[doc = "RFID射频读写设备"]
pub struct RFIDDevice {
    mfrc522: Mfrc522<
        Spi<
            SPI2,
            Spi2NoRemap,
            (
                gpiob::PB13<Alternate<PushPull>>,
                gpiob::PB14<Input<Floating>>,
                gpiob::PB15<Alternate<PushPull>>,
            ),
            u8,
        >,
        gpiob::PB12<Output<PushPull>>,
    >,
    delay: SysDelay,
}

impl RFIDDevice {
    pub fn new(
        spi2: SPI2,
        pb12: gpio::PB12,
        pb13: gpio::PB13,
        pb14: gpio::PB14,
        pb15: gpio::PB15,
        clocks: rcc::Clocks,
        crh: &mut gpio::Cr<'B', true>,
    ) -> Self {
        // SPI configuration
        let nss = pb12.into_push_pull_output(crh);
        let sck = pb13.into_alternate_push_pull(crh);
        let miso = pb14.into_floating_input(crh);
        let mosi = pb15.into_alternate_push_pull(crh);

        let spi_mode = Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        };
        let spi = Spi::spi2(spi2, (sck, miso, mosi), spi_mode, 400.kHz(), clocks);

        let mfrc522 = Mfrc522::new(spi, nss).unwrap();

        // delay
        let cp = pac::Peripherals::ctake().unwrap();
        let delay = cp.SYST.delay(&clocks);

        Self { mfrc522, delay }
    }

    pub fn read(&mut self, buz: &mut buzzer::BuzzerDevice) -> Strfmt {
        let mut s = Strfmt::new();
        if let Ok(atqa) = self.mfrc522.reqa() {
            if let Ok(uid) = self.mfrc522.select(&atqa) {
                buz.on();

                s.add_slice(uid.as_bytes());
                self.delay.delay_ms(500_u16);

                buz.off();
            }
        }
        s
    }
}
