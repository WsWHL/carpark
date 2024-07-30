#![allow(dead_code)]

use crate::base::fonts;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X13, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::mode::BufferedGraphicsMode;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use stm32f1::stm32f103::I2C1;
use stm32f1xx_hal::gpio::{self, Alternate, OpenDrain};
use stm32f1xx_hal::i2c::{BlockingI2c, DutyCycle, Mode};
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::{afio, rcc};

#[doc = "oled屏幕显示设备"]
pub struct OLEDDevice {
    display: Ssd1306<
        I2CInterface<
            BlockingI2c<
                I2C1,
                (
                    gpio::PB8<Alternate<OpenDrain>>,
                    gpio::PB9<Alternate<OpenDrain>>,
                ),
            >,
        >,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >,
}

impl OLEDDevice {
    pub fn new(
        ic21: I2C1,
        pb8: gpio::PB8,
        pb9: gpio::PB9,
        clocks: rcc::Clocks,
        crh: &mut gpio::Cr<'B', true>,
        mapr: &mut afio::MAPR,
    ) -> Self {
        let scl = pb8.into_alternate_open_drain(crh);
        let sda = pb9.into_alternate_open_drain(crh);

        let i2c = BlockingI2c::i2c1(
            ic21,
            (scl, sda),
            mapr,
            Mode::fast(100.kHz(), DutyCycle::Ratio2to1),
            clocks,
            1000,
            10,
            1000,
            1000,
        );

        let interface = I2CDisplayInterface::new(i2c);
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        display.init().unwrap();

        Self { display }
    }

    pub fn text(&mut self, s: &str, point: Point) {
        let style = MonoTextStyleBuilder::new()
            .font(&FONT_6X13)
            .text_color(BinaryColor::On)
            .build();

        Text::with_baseline(s, point, style, Baseline::Top)
            .draw(&mut self.display)
            .unwrap();
    }

    pub fn text_pixel(&mut self, string: &str, line: u32, column: u32) {
        for (i, c) in string.chars().enumerate() {
            let (x_offset, y_offset) = ((column - 1 + i as u32) * 16, (line - 1) * 16);

            if let Ok(bytes) = fonts::get_zh_font(c) {
                for x in 0..16u32 {
                    let b = bytes[x as usize];
                    for y in 0..8u32 {
                        if b & (0x01 << y) == 0 {
                            self.display.set_pixel(x + x_offset, y + y_offset, false);
                        } else {
                            self.display.set_pixel(x + x_offset, y + y_offset, true);
                        }
                    }
                }

                for x in 0..16u32 {
                    let b = bytes[(x + 16) as usize];
                    for y in 0..8u32 {
                        if b & (0x01 << y) == 0 {
                            self.display
                                .set_pixel(x + x_offset, y + y_offset + 8, false);
                        } else {
                            self.display.set_pixel(x + x_offset, y + y_offset + 8, true);
                        }
                    }
                }
            }
        }
    }

    pub fn flash(&mut self) {
        self.display.flush().unwrap();
    }

    pub fn clear(&mut self) {
        self.display.clear(BinaryColor::Off).unwrap();
    }
}
