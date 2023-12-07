use std::collections::{BinaryHeap, HashMap};

fn process_hand(hand: &str) -> Vec<usize> {
    let mut counts: HashMap<char, usize> = HashMap::new();

    for char in hand.chars() {
        let _ = *counts
            .entry(char)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut counts_sorted = counts.values().collect::<BinaryHeap<&usize>>();

    let max = *counts_sorted.pop().unwrap();
    let second_max = *counts_sorted.pop().unwrap_or(&0);
    let hand_score = if max <= 2 && second_max != 2 {max} else if max == 2 || (max == 3 && second_max != 2) {max + 1} else {max + 2};
    let mut hand_vec = vec![hand_score];
    hand_vec.extend(hand.chars().map(|char| char.to_digit(10).unwrap_or(
    match char {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 99,
    }) as usize));

    hand_vec
}

pub fn run() {
    let mut hands = include_str!("input")
        .lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .map(|l| (process_hand(l[0]), l[1].parse::<usize>().unwrap()))
        .collect::<Vec<(Vec<usize>, usize)>>();
    
    hands.sort_by(|a, b| a.0.cmp(&b.0));

    let part1: usize = hands.iter().map(|hand| hand.1)
    .enumerate()
    .map(|(rank, score)| score * (rank + 1)).sum();
    for hand in hands{
        println!("{:?}", hand)
    }
    println!("{}", part1);
}
