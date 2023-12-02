use std::cmp::max;

pub fn run(){
    let part1: usize = include_str!("input")
    .lines().map(
        |l| l.split(":").collect::<Vec<&str>>()[1]
        .split(";")
        .all(|pull| pull.split(",")
            .map(|num_color| 
                num_color.split(" ").skip(1).collect::<Vec<&str>>()
            )
            .all(
                |pair| match (pair[0].parse::<usize>().unwrap(), pair[1]) {
                    (1..=12, "red") => true,
                    (1..=13, "green") => true,
                    (1..=14, "blue") => true,
                    _ => false
                }
            )
        )
    ).enumerate().filter(|(_, valid)| *valid).map(|(game_number, _)| game_number + 1).sum();

    println!("{}", part1);

    let part2: usize = include_str!("input")
    .lines().map(
        |l| l.split(":").collect::<Vec<&str>>()[1]
        .split(";")
        .flat_map(|pull| pull.split(",")
            .map(|num_color| 
                num_color.split(" ").skip(1).collect::<Vec<&str>>()
            )
        )
        .fold( [0,0,0], 
            |current, pair| match (pair[0].parse::<usize>().unwrap(), pair[1]) {
                (num, "red") => [max(current[0], num), current[1], current[2]],
                (num, "green") => [current[0], max(current[1], num), current[2]],
                (num, "blue") => [current[0], current[1], max(current[2], num)],
                _ => current
            }
        ).into_iter().product::<usize>()
    ).sum();

    println!("{}", part2)
}
