fn process_history(history: Vec<i32>) -> i32 {
    let mut histories = vec![history.clone()];
    while !histories[histories.len() - 1].iter().all(|v| *v == 0) {
        histories.push(
            histories[histories.len() - 1]
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect(),
        );
    }

    return histories.iter().map(|h| h[h.len() - 1]).sum();
}

fn process_history_2(history: Vec<i32>) -> i32 {
    let mut histories = vec![history.clone()];
    while !histories[histories.len() - 1].iter().all(|v| *v == 0) {
        histories.push(
            histories[histories.len() - 1]
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect(),
        );
    }

    return histories
        .iter()
        .map(|h| h[0])
        .enumerate()
        .map(|(i, v)| if i % 2 == 0 { v } else { -1 * v })
        .sum();
}

pub fn run() {
    let input = include_str!("input").lines().map(|l| {
        l.split(" ")
            .map(|numstring| numstring.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });
    let predictions = input.map(process_history);
    let part1: i32 = predictions.sum();

    println!("{}", part1);

    let input = include_str!("input").lines().map(|l| {
        l.split(" ")
            .map(|numstring| numstring.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });
    let predictions = input.map(process_history_2);
    let part2: i32 = predictions.sum();

    println!("{}", part2)
}
