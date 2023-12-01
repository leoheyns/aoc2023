use chrono::{FixedOffset, Utc, Datelike};
use std::env;
#[path = "day1/day1.rs"]
mod day1;

fn run_day(day: u32){
    match day{
        1=>day1::run(),
        _=>println!("Soepkip! die dag bestaat niet",)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let timezone = FixedOffset::west_opt(5*3600).unwrap();
    let now = Utc::now();
    let datetime = now.with_timezone(&timezone);
    let mut day = datetime.date_naive().day();

    
    if args.len() == 2{
        day = args[1].parse::<u32>().unwrap()
    }
    if day < 25 && day > 0{
        run_day(day);
    }
}