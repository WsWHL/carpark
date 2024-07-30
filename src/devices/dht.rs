use dht11::Dht11;
use stm32f1xx_hal::{
    gpio::{self, OpenDrain, Output},
    rcc,
    timer::{SysDelay, SysTimerExt},
};

use crate::base::pac;

#[doc = "DHT11温湿度传感器设备"]
pub struct DHTDevice {
    dht: Dht11<gpio::PB1<Output<OpenDrain>>>,
    delay: SysDelay,
}

impl DHTDevice {
    pub fn new(pa7: gpio::PB1, clocks: rcc::Clocks, crl: &mut gpio::Cr<'B', false>) -> Self {
        let pin = pa7.into_open_drain_output(crl);

        let dht = Dht11::new(pin);

        // delay
        let cp = pac::Peripherals::ctake().unwrap();
        let delay = cp.SYST.delay(&clocks);

        Self { dht, delay }
    }

    pub fn read(&mut self) -> (i16, u16) {
        if let Ok(meas) = self.dht.perform_measurement(&mut self.delay) {
            return (meas.temperature, meas.humidity);
        }

        (0, 0)
    }
}
