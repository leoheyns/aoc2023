use std::iter::zip;

pub fn run() {
    let mut input = include_str!("input").lines().map(|l| {
        l.split(":")
            .skip(1)
            .next()
            .unwrap()
            .split_whitespace()
            .map(|numstring| numstring.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    });
    let races = zip(input.next().unwrap(), input.next().unwrap());

    let part1: usize = races
        .map(|(time, record)| {
            (1..time)
                .filter(|speed| speed * (time - speed) > record)
                .count()
        })
        .product();

    println!("{}", part1);

    let mut input = include_str!("input").lines().map(|l| {
        l.split(":")
            .skip(1)
            .next()
            .unwrap()
            .replace(" ", "")
            .parse::<usize>()
            .unwrap()
    });

    let part2: usize = (|(time, record)| {
        (1..time)
            .filter(|speed| speed * (time - speed) > record)
            .count()
    })((input.next().unwrap(), input.next().unwrap()));

    println!("{}", part2);
}
