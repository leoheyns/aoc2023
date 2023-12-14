use std::cmp::min;

fn find_symetries(pattern: Vec<Vec<char>>, differences_expected: usize) -> usize {
    for i in 0..pattern.len() - 1 {
        let mut differences = 0;
        for j in 0..min(pattern.len() - i - 1, i + 1) {
            differences += pattern[i - j]
                .iter()
                .zip(pattern[i + j + 1].iter())
                .filter(|(l, r)| *l != *r)
                .count();
        }
        if differences == differences_expected {
            return 100 * (i + 1);
        }
    }

    for i in 0..pattern[0].len() - 1 {
        let mut differences = 0;
        for j in 0..min(pattern[0].len() - i - 1, i + 1) {
            differences += pattern
                .iter()
                .map(|l| l[i - j])
                .zip(pattern.iter().map(|l| l[i + j + 1]))
                .filter(|(l, r)| *l != *r)
                .count();
        }
        if differences == differences_expected {
            return i + 1;
        }
    }
    return 0;
}

pub fn run() {
    let input: Vec<Vec<Vec<char>>> = include_str!("input")
        .split("\n\n")
        .map(|pat| {
            pat.lines()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect();

    let result = input
        .iter()
        .map(|pat| find_symetries(pat.clone(), 0))
        .sum::<usize>(); //collect::<Vec<usize>>();

    let result2 = input
        .iter()
        .map(|pat| find_symetries(pat.clone(), 1))
        .sum::<usize>(); //collect::<Vec<usize>>();

    println!("{:?}", result);
    println!("{:?}", result2);
}
