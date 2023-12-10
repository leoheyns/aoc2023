use std::collections::HashMap;

pub fn run() {
    let mut networkmap: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut input = include_str!("input").split("\n\n");

    let instructions = input.next().unwrap();
    for line in input.next().unwrap().lines() {
        networkmap.insert(&line[..3], (&line[7..10], &line[12..15]));
    }

    // let mut current_location = "AAA";
    // let mut current_instruction = 0;
    // while current_location != "ZZZ"{
    //     if instructions.chars().collect::<Vec<char>>()[current_instruction % instructions.len()] == 'L'{
    //         current_location = networkmap.get(current_location).unwrap().0
    //     } else {
    //         current_location = networkmap.get(current_location).unwrap().1
    //     }

    //     current_instruction += 1;
    // }

    // println!("{}", current_instruction);

    //part2

    let mut current_locations: Vec<&str> = networkmap
        .keys()
        .filter(|location| location.chars().last().unwrap() == 'A')
        .map(|location| *location)
        .collect();
    let mut current_instruction = 0;
    let mut end_location_times: Vec<Vec<usize>> = vec![vec![]; current_locations.len()];

    for _ in 0..100000 {
        // while !current_locations.iter().all(|location| location.chars().last().unwrap() == 'Z'){
        for i in 0..current_locations.len() {
            if current_locations[i].chars().last().unwrap() == 'Z' {
                // let last_z = if end_location_times[i].len() == 0 {
                //     0
                // } else {
                //     end_location_times[i][end_location_times[i].len() - 1]
                // };
                end_location_times[i].push(current_instruction);
            }
            if instructions.chars().collect::<Vec<char>>()[current_instruction % instructions.len()]
                == 'L'
            {
                current_locations[i] = networkmap.get(current_locations[i]).unwrap().0
            } else {
                current_locations[i] = networkmap.get(current_locations[i]).unwrap().1
            }
        }
        current_instruction += 1;
        // if current_instruction % 10000 == 0{
        //     println!("{}", current_instruction)
        // }
    }
    for mut g in end_location_times {
        for i in (1..g.len()).rev() {
            g[i] = g[i] - g[i - 1];
        }
    }
}
