use chrono::{Datelike, FixedOffset, Utc};
use std::env;
#[path = "day01/day01.rs"]
mod day01;
#[path = "day02/day02.rs"]
mod day02;
#[path = "day03/day03.rs"]
mod day03;
#[path = "day04/day04.rs"]
mod day04;

fn run_day(day: u32) {
    match day {
        01 => day01::run(),
        02 => day02::run(),
        03 => day03::run(),
        04 => day04::run(),
        _ => println!("Soepkip! die dag bestaat niet",),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let timezone = FixedOffset::west_opt(5 * 3600).unwrap();
    let now = Utc::now();
    let datetime = now.with_timezone(&timezone);
    let mut day = datetime.date_naive().day();

    if args.len() == 2 {
        day = args[1].parse::<u32>().unwrap()
    }
    if day < 25 && day > 0 {
        run_day(day);
    }
}
