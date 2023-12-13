use std::cmp::{max, min};

pub fn run() {
    let grid: Vec<Vec<bool>> = include_str!("input")
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect();

    // part1 jank
    // let mut i = 0;

    // while i < grid.len(){
    //     if !grid[i].iter().any(|b|*b){
    //         grid.insert(i, vec![false; grid[0].len()]);
    //         i+=1;
    //     }
    //     i += 1
    // }

    // let mut j = 0;
    // while j < grid[0].len(){
    //     if !grid.iter().map(|l| l[j]).any(|b|b){

    //         for i in 0..grid.len(){
    //             grid[i].insert(j, false)
    //         }
    //         j += 1;
    //     }
    //     j += 1;
    // }

    // let mut i = 0;

    let empty_lines: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter(|(_, l)| !l.iter().any(|b| *b))
        .map(|(i, _)| i)
        .collect();
    let mut empty_columns: Vec<usize> = vec![];

    for j in 0..grid[0].len() {
        if !grid.iter().map(|l| l[j]).any(|b| b) {
            empty_columns.push(j)
        }
    }

    let mut galaxies: Vec<(i32, i32)> = vec![];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] {
                galaxies.push((i as i32, j as i32))
            }
        }
    }

    let mut result: usize = 0;
    let expansion: usize = 999999;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let from = galaxies[i];
            let to = galaxies[j];
            result += ((from.0 - to.0).abs() + (from.1 - to.1).abs()) as usize;
            for line in empty_lines.clone() {
                if line > min(from.0 as usize, to.0 as usize)
                    && line < max(from.0 as usize, to.0 as usize)
                {
                    result += expansion
                }
            }

            for col in empty_columns.clone() {
                if col > min(from.1 as usize, to.1 as usize)
                    && col < max(from.1 as usize, to.1 as usize)
                {
                    result += expansion
                }
            }
        }
    }
    // for i in 0.. grid.len(){
    //     // println!("{:?}", grid[i]);
    //     for j in 0..grid[0].len(){
    //         print!("{:}", if grid[i][j] {'#'} else {'.'})
    //     }
    //     println!()
    // }

    println!("{}", result);

    println!("{} {}", empty_lines.len(), empty_columns.len())
}
