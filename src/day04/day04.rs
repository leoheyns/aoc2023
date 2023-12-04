fn check_card(card: Vec<Vec<usize>>) -> usize {
    let winners = card[0].clone();
    let draws = card[1].clone();

    let wins = draws
        .into_iter()
        .filter(|draw| winners.contains(&draw))
        .count();

    return wins;
}

pub fn run() {
    let wins: Vec<usize> = include_str!("input")
        .lines()
        .map(|line| {
            line.split(": ")
                .last()
                .unwrap()
                .split(" | ")
                .map(|nums| {
                    nums.split(" ")
                        .filter(|s| s.len() > 0)
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .map(check_card)
        .collect();

    let base: usize = 2;
    let part1: usize = wins
        .clone()
        .into_iter()
        .map(|w| if w == 0 { 0 } else { base.pow((w - 1) as u32) })
        .sum();
    println!("{}", part1);

    let mut cards = vec![1; wins.len()];

    for (i, wins) in wins.into_iter().enumerate() {
        for j in (i + 1)..std::cmp::min(i + 1 + wins, cards.len()) {
            cards[j] += cards[i]
        }
    }

    let part2: usize = cards.into_iter().sum();

    println!("{}", part2)
}
