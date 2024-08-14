#![no_std]
#![no_main]

mod base;
mod devices;
mod parking;

use async_embedded::{task, unsync::Channel};
use base::strings;
use base::utils;
use core::fmt::Write;
use cortex_m::asm;
use cortex_m_rt::entry;
use heapless::String;
use panic_halt as _;
use parking::park;
use stm32f1xx_hal::time::ms;
use time::Duration;

static mut BUF: String<64> = String::<64>::new();

#[entry]
fn main() -> ! {
    // 初始化系统
    let m = park::Modules::new();
    let mut park = park::Parking::<2>::new(m.led);

    // 多任务通信Channel
    static mut C: Channel<i64> = Channel::new();
    static mut V: Channel<&str> = Channel::new();

    // 初始化时间
    let mut ds = m.ds;
    ds.init();

    task::spawn(async move {
        let mut sg = m.sg;
        loop {
            let uid = unsafe { C.recv().await };
            if uid > 0 {
                // 启动闸机
                sg.on();
            }

            task::r#yield().await;
        }
    });

    task::spawn(async move {
        let mut voice = m.voice;
        loop {
            let msg = unsafe { V.recv().await };
            if msg != "" {
                // 语音播报
                unsafe {
                    voice.broadcast(BUF.as_str());
                    BUF.clear();
                };
                voice.broadcast(msg);
            }

            task::r#yield().await;
        }
    });

    task::block_on(async {
        let mut oled = m.oled;
        let mut rf = m.rfid;
        let mut buz = m.buzzer;
        let mut dht = m.dht;
        loop {
            let now = ds.get_time();
            let ts = now.unix_timestamp();

            // 清除屏幕显示内容
            oled.clear();

            // 显示时间
            let t = utils::format_time(now);
            oled.text_small_pixel(t.as_str(), 1, 1);

            // 显示卡号
            if let Ok(uid) = rf.read(&mut buz) {
                let id = utils::parse_int::<i64>(uid.as_str()).unwrap();
                // 扫描卡片
                let mut card = format!("{}", uid);
                let mut tips = "欢迎光临！";

                match park.scanning(id, ts) {
                    Ok(in_ts) => {
                        if ts > in_ts {
                            let d = Duration::new(ts - in_ts, 0);
                            let amount = if d.whole_seconds() % 10 == 0 {
                                d.whole_seconds() / 10
                            } else {
                                d.whole_seconds() / 10 + 1
                            };
                            if d.whole_minutes() > 0 {
                                card = format!(
                                    "{} {}分{}秒 {}元",
                                    uid,
                                    d.whole_minutes(),
                                    d.whole_seconds() % 60,
                                    amount
                                )
                            } else {
                                card = format!("{} {}秒 {}元", uid, d.whole_seconds(), amount);
                            }
                            tips = "祝你一路顺风！";
                        }

                        unsafe {
                            write!(BUF, "{}", card.as_str()).unwrap();
                            C.send(id).await
                        }
                    }
                    Err(e) => {
                        if let park::ScanErrors::ParkingLimit = e {
                            tips = "车位已满！";
                        }
                    }
                }

                unsafe {
                    V.send(tips).await;
                }

                oled.text_small_pixel(card.as_str(), 2, 1);
                oled.text_pixel(tips, 3, 1);
            } else {
                let n = park.get_idle();
                if n > 0 {
                    let idle = format!("剩余{}个车位", n);
                    oled.text_pixel(idle.as_str(), 3, 1);
                } else {
                    oled.text_pixel("车位已满", 3, 1);
                }
            }

            // 读取温湿度
            let (temp, humi) = dht.read();
            if temp > 0 || humi > 0 {
                let th = format!("温度:{}.{}℃ 湿度:{}%", temp / 10, temp % 10, humi / 10);
                oled.text_small_pixel(th.as_str(), 4, 1);
            }

            oled.flash();
            asm::delay(ms(50).ticks());
            task::r#yield().await;
        }
    })
}
