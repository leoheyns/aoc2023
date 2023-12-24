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
#[path = "day05/day05.rs"]
mod day05;
#[path = "day06/day06.rs"]
mod day06;
#[path = "day07/day07.rs"]
mod day07;
#[path = "day08/day08.rs"]
mod day08;
#[path = "day09/day09.rs"]
mod day09;
#[path = "day10/day10.rs"]
mod day10;
#[path = "day11/day11.rs"]
mod day11;
#[path = "day12/day12.rs"]
mod day12;
#[path = "day13/day13.rs"]
mod day13;
#[path = "day14/day14.rs"]
mod day14;
#[path = "day15/day15.rs"]
mod day15;
#[path = "day16/day16.rs"]
mod day16;
#[path = "day17/day17.rs"]
mod day17;
#[path = "day18/day18.rs"]
mod day18;
#[path = "day19/day19.rs"]
mod day19;
#[path = "day20/day20.rs"]
mod day20;
#[path = "day21/day21.rs"]
mod day21;
#[path = "day22/day22.rs"]
mod day22;
#[path = "day23/day23.rs"]
mod day23;
#[path = "day24/day24.rs"]
mod day24;

fn run_day(day: u32) {
    match day {
        19 => day19::run(),
        08 => day08::run(),
        24 => day24::run(),
        09 => day09::run(),
        12 => day12::run(),
        10 => day10::run(),
        11 => day11::run(),
        13 => day13::run(),
        22 => day22::run(),
        23 => day23::run(),
        01 => day01::run(),
        05 => day05::run(),
        06 => day06::run(),
        21 => day21::run(),
        17 => day17::run(),
        02 => day02::run(),
        18 => day18::run(),
        03 => day03::run(),
        15 => day15::run(),
        16 => day16::run(),
        14 => day14::run(),
        04 => day04::run(),
        07 => day07::run(),
        20 => day20::run(),
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
