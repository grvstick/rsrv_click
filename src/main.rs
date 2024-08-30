use chrono::prelude::{DateTime, Local, SecondsFormat};
use mouse_rs::{types::keys::Keys, Mouse};
use rsntp::SntpClient;
use serde::Deserialize;
use std::{
    fmt::Debug, io::{stdout, Write}, thread::sleep, time::Duration
};

#[derive(Deserialize, Debug)]
struct DT {
    tgt: String,
    delay: f64,
}


fn sync_current_time() -> f64 {
    let client = SntpClient::new();
    let result = client.synchronize("pool.ntp.org").unwrap();

    println!("{:?}", result);
    let offset = result.clock_offset().as_secs_f64();
    let rtd = result.round_trip_delay().as_secs_f64();
    offset + rtd / 2.0
}

fn main() {
    let mouse = Mouse::new();

    let offset = sync_current_time();
    let config = std::fs::read_to_string("config.toml").expect("unable to read config.toml");
    let dt: DT = toml::from_str(&config).expect("toml parsing error");

    println!("Add manual delay: {}s", dt.delay);
    let offset = offset + dt.delay;
    let tgt_dt = [dt.tgt.trim(), "+09:00"].join("");
    let mut tgt_dt: DateTime<Local> = tgt_dt.parse().expect("");

    println!("Target Time(User)\t{}", tgt_dt.to_rfc3339_opts(SecondsFormat::Micros, false));
    if offset > 0.0 {
        tgt_dt -= Duration::from_secs_f64(offset);
    } else {
        tgt_dt += Duration::from_secs_f64(-offset);
    }

    println!("Target Time(Cal)\t{}", tgt_dt.to_rfc3339_opts(SecondsFormat::Micros, false));

    let delta = (tgt_dt - Local::now()).num_seconds();
    if delta < 0 {
        println!("Target time already reached! aborting...");
        sleep(Duration::from_secs(5));
        return;
    }
    let mut delay_ms = 0;
    let mut remaining_sec;
    let mut remaining_sec_buf = 0;
    let mut print_remaining;
    let mut abort = false;

    loop {
        print_remaining = true;
        let dt = Local::now();
        let delta = tgt_dt - dt;
        let remaining_ms = delta.num_milliseconds();
        remaining_sec = delta.num_seconds();

        if remaining_ms > 5000 {
            delay_ms = 1000;
        } else if remaining_ms > 0 {
            delay_ms = 10;
            print_remaining = remaining_sec_buf != remaining_sec;
            if print_remaining {
                remaining_sec_buf = remaining_sec;
            }
        } else {
            abort = true;
            mouse.click(&Keys::LEFT).unwrap();
            mouse.release(&Keys::LEFT).unwrap();
        }

        if print_remaining {
            print!("Current Time:\t\t{}", dt.to_rfc3339_opts(SecondsFormat::Micros, false));
        }

        if abort {
            println!();
            println!("clicked within {} ms", remaining_ms as f32 / 1000.0);
            sleep(Duration::from_secs(20));
            return;
        }

        print!("\r");
        stdout().flush().unwrap();
        sleep(Duration::from_millis(delay_ms as u64));
    }
}
