use chrono::prelude::{DateTime, Local};
use mouse_rs::{types::keys::Keys, Mouse};
use rsntp::SntpClient;
use serde::Deserialize;
use std::{thread::sleep, time::Duration};

#[derive(Deserialize, Debug)]
struct DT {
    dt: String,
}

// pub fn get_iso_time() -> String {
//     let now = SystemTime::now();
//     let now: DateTime<Utc> = now.into();
//     now.to_rfc3339()
// }

fn sync_current_time() -> f64 {
    let client = SntpClient::new();
    let result = client.synchronize("pool.ntp.org").unwrap();
    // let local_time: DateTime<Local> =
    //     DateTime::from(result.datetime().into_chrono_datetime().unwrap());
    let offset = result.clock_offset().as_secs_f64();
    println!("Time sync done, offset is {}", offset);
    offset
}

fn main() {
    let mouse = Mouse::new();

    let offset = sync_current_time();
    let config = std::fs::read_to_string("config.toml").expect("unable to read config.toml");
    let tgt_dt: DT = toml::from_str(&config).expect("toml parsing error");

    let tgt_dt = [tgt_dt.dt.trim(), "+09:00"].join("");
    let mut tgt_dt: DateTime<Local> = tgt_dt.parse().expect("");

    println!("Target time {}", tgt_dt);
    if offset < 0.0 {
        tgt_dt -= Duration::from_secs_f64(-offset);
    } else {
        tgt_dt += Duration::from_secs_f64(offset);
    }

    println!("Calibrated target time {}", tgt_dt);
    // println!("{:?}", dt - time);

    let delta = (tgt_dt - Local::now()).num_seconds();
    if delta < 0 {
        println!("Target time already reached! aborting...");
        sleep(Duration::from_secs(5));
        return;
    }
    let mut delay_ms;
    let mut remaining_sec;
    let mut remaining_sec_buf = 0;
    let mut print_remaining;

    loop {
        print_remaining = true;
        let delta = tgt_dt - Local::now();
        let remaining_ms = delta.num_milliseconds();
        remaining_sec = delta.num_seconds();

        if remaining_ms > 70_000 {
            delay_ms = 60_000;
            if remaining_ms % 60_000 > 10_000 {
                delay_ms += remaining_ms % 60_000 - 10_000;
            }
        } else if remaining_ms > 20_000 {
            delay_ms = 10_000;
        } else if remaining_ms > 0 {
            delay_ms = 10;
            print_remaining = remaining_sec_buf != remaining_sec;
            if print_remaining {
                remaining_sec_buf = remaining_sec;
            }
        } else {
            mouse.click(&Keys::LEFT).unwrap();
            mouse.release(&Keys::LEFT).unwrap();
            println!("clicked within {} ms", remaining_ms as f32 / 1000.0);
            sleep(Duration::from_secs(20));
            return;
        }

        if print_remaining {
            if remaining_sec > 60 {
                let remaining_min = remaining_sec / 60;
                println!("{} minute(s) remaining...", remaining_min);
            } else {
                println!("{} second(s) remaining...", remaining_ms as f32 / 1000.0);
            }
        }
        sleep(Duration::from_millis(delay_ms as u64));
    }
}
