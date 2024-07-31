#![no_std]
#![no_main]

mod base;
mod devices;
mod parking;

use async_embedded::{task, unsync::Channel};
use base::sys;
use base::utils;
use cortex_m::asm;
use cortex_m_rt::entry;
use embedded_graphics::prelude::Point;
use no_std_strings::str64;
use panic_halt as _;
use parking::park;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::time::ms;
use time::Duration;

#[entry]
fn main() -> ! {
    let s = sys::Sys::new();
    let time = s.time_rtc();
    let mut delay = s.get_delay();

    // 初始化系统
    let (mut oled, mut rf, mut sg, led, mut buz, mut dht, mut voice) = park::init_devices();
    let mut park = park::Parking::<2>::new(led);

    // 多任务通信Channel
    static mut C: Channel<i64> = Channel::new();
    static mut V: Channel<&str> = Channel::new();

    task::spawn(async move {
        loop {
            let uid = unsafe { C.recv().await };
            if uid > 0 {
                // 启动闸机
                sg.put_up();

                delay.delay_ms(1000_u16);

                // 关闭闸机
                sg.put_down();
            }

            task::r#yield().await;
        }
    });

    task::spawn(async move {
        loop {
            let msg = unsafe { V.recv().await };
            if msg != "" {
                // 语音播报
                voice.broadcast(msg);
            }

            task::r#yield().await;
        }
    });

    task::block_on(async {
        loop {
            let ts = time.current_time() as i64;
            let t = utils::format_unix_time(ts);
            let uid = rf.read(&mut buz);

            // 清除屏幕显示内容
            oled.clear();

            // 显示时间
            oled.text(t.as_str(), Point::zero());

            // 显示卡号
            if uid.is_empty() {
                let n = park.get_idle();
                if n > 0 {
                    let idle = no_std_strings::str_format!(str64, "剩余{}个车位", n);
                    oled.text_pixel(idle.as_str(), 3, 1);
                } else {
                    oled.text_pixel("车位已满", 3, 1);
                }
            } else {
                let id = utils::parse_int::<i64>(uid.as_str()).unwrap();
                // 扫描卡片
                let mut card = no_std_strings::str_format!(str64, "{}", uid);
                let mut tips = "欢迎光临！";

                match park.scanning(id, ts) {
                    Ok(in_ts) => {
                        unsafe { C.send(id).await }

                        if ts > in_ts {
                            let d = Duration::new(ts - in_ts, 0);
                            card = no_std_strings::str_format!(str64, "{} {}", uid, d);
                            tips = "祝你一路顺风！";
                        }
                    }
                    Err(e) => {
                        if let park::ScanErrors::ParkingLimit = e {
                            tips = "车位已满！";
                        }
                    }
                }

                unsafe { V.send(tips).await }

                oled.text(card.as_str(), Point::new(0, 15));
                oled.text_pixel(tips, 3, 1);
            }

            // 读取温湿度
            let (temp, humi) = dht.read();
            if temp > 0 || humi > 0 {
                let th = no_std_strings::str_format!(
                    str64,
                    "temp:{}.{}C  humi:{}%",
                    temp / 10,
                    temp % 10,
                    humi / 10
                );
                oled.text(th.as_str(), Point::new(0, 50));
            }

            oled.flash();
            asm::delay(ms(50).ticks());
            task::r#yield().await;
        }
    })
}
