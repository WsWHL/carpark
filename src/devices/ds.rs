use ds323x::{ic::DS3231, interface::I2cInterface, DateTimeAccess, Ds323x, NaiveDate};
use stm32f1::stm32f103::I2C2;
use stm32f1xx_hal::{
    gpio::{self, Alternate, OpenDrain},
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::_fugit_RateExtU32,
    rcc,
};
use time::OffsetDateTime;

pub struct DSDevice {
    rtc: Ds323x<
        I2cInterface<
            BlockingI2c<
                I2C2,
                (
                    gpio::PB10<Alternate<OpenDrain>>,
                    gpio::PB11<Alternate<OpenDrain>>,
                ),
            >,
        >,
        DS3231,
    >,
}

impl DSDevice {
    pub fn new(
        i2c2: I2C2,
        pb10: gpio::PB10,
        pb11: gpio::PB11,
        clocks: rcc::Clocks,
        crh: &mut gpio::Cr<'B', true>,
    ) -> Self {
        let scl = pb10.into_alternate_open_drain(crh);
        let sda = pb11.into_alternate_open_drain(crh);

        let i2c = BlockingI2c::i2c2(
            i2c2,
            (scl, sda),
            Mode::fast(100.kHz(), DutyCycle::Ratio2to1),
            clocks,
            1000,
            10,
            1000,
            1000,
        );

        let rtc = Ds323x::new_ds3231(i2c);

        Self { rtc }
    }

    pub fn init(&mut self) {
        if self.rtc.running().unwrap() {
            return;
        }

        let begin = NaiveDate::from_ymd_opt(2024, 8, 6)
            .unwrap()
            .and_hms_opt(15, 53, 00)
            .unwrap();
        self.rtc.set_datetime(&begin).unwrap();
    }

    pub fn get_time(&mut self) -> OffsetDateTime {
        let dt = self.rtc.datetime().unwrap();
        let ts = dt.and_utc().timestamp();
        OffsetDateTime::from_unix_timestamp(ts).unwrap()
    }
}
