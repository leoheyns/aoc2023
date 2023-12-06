use std::cmp::Ordering;

fn process_seed(seed: &usize, xtoymaps: &Vec<Vec<Vec<usize>>>) -> usize {
    let mut current_num = *seed;

    for xtoymap in xtoymaps {
        current_num = match xtoymap.binary_search_by(|range| {
            if current_num < range[1] {
                Ordering::Greater
            } else if current_num >= range[1] + range[2] {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }) {
            Ok(index) => current_num + xtoymap[index][0] - xtoymap[index][1],
            Err(_) => current_num,
        };
    }

    return current_num;
}

pub fn run() {
    let mut input = include_str!("testinput").split("\n\n");
    let seeds = input
        .next()
        .unwrap()
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .map(|numstring| numstring.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut xtoymaps = input
        .map(|mp| {
            mp.lines()
                .skip(1)
                .map(|l| {
                    l.split(" ")
                        .map(|numstring| numstring.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .collect::<Vec<Vec<Vec<usize>>>>();

    for i in 0..xtoymaps.len() {
        xtoymaps[i].sort_by_key(|range| range[1]);
    }

    let result = seeds
        .iter()
        .map(|seed| process_seed(seed, &xtoymaps))
        .min()
        .unwrap();

    println!("{}", result);

    let mut seed_ranges = vec![(0, 0); seeds.len() / 2];

    for i in 0..(seeds.len() / 2) {
        seed_ranges[i] = (seeds[i * 2], seeds[i * 2 + 1])
    }

    let expanded_seeds = seed_ranges
        .iter()
        .flat_map(|range| range.0..(range.0 + range.1));

    let result: usize = expanded_seeds
        .map(|seed| process_seed(&seed, &xtoymaps))
        .min()
        .unwrap();

    println!("result: {}", result);
}
