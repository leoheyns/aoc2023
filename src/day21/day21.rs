use std::vec;

fn _printgrid(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        println!()
    }
}

fn count_subgrid(grid: &Vec<Vec<char>>, xbig: usize, ybig: usize) -> usize {
    let mut result: usize = 0;
    for ysmall in 0..131 {
        for xsmall in 0..131 {
            if grid[ybig * 131 + ysmall][xbig * 131 + xsmall] == 'O' {
                result += 1;
            }
        }
    }
    return result;
}

// fn get_possible_states(grid, starting_coord)

pub fn run() {
    let input_grid: Vec<Vec<char>> = include_str!("input")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let mut grid = vec![vec!['.'; input_grid[0].len() * 5]; input_grid.len() * 5];

    for ybig in 0..5 {
        for xbig in 0..5 {
            for ysmall in 0..input_grid.len() {
                for xsmall in 0..input_grid[0].len() {
                    if input_grid[ysmall][xsmall] == 'S' && !(ybig == 2 && xbig == 2) {
                        grid[ybig * 131 + ysmall][xbig * 131 + xsmall] = '.';
                    } else {
                        grid[ybig * 131 + ysmall][xbig * 131 + xsmall] = input_grid[ysmall][xsmall];
                    }
                }
            }
        }
    }
    // _printgrid(&grid);

    for _ in 0..(2 * 131 + 65) {
        // printgrid(&grid);
        // println!();
        let mut next_grid = grid.clone();
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == 'O' || grid[y][x] == 'S' {
                    for (yoffset, xoffset) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        if (y as i32 + yoffset) >= 0
                            && y as i32 + yoffset < grid.len() as i32
                            && (x as i32 + xoffset) >= 0
                            && x as i32 + xoffset < grid[0].len() as i32
                        {
                            if grid[(y as i32 + yoffset) as usize][(x as i32 + xoffset) as usize]
                                == '.'
                            {
                                next_grid[(y as i32 + yoffset) as usize]
                                    [(x as i32 + xoffset) as usize] = 'O';
                            }
                        }
                        next_grid[y][x] = '.';
                    }
                }
            }
        }
        grid = next_grid;
    }

    let dist_from_center = 202300;

    let odds = count_subgrid(&grid, 2, 2) * (dist_from_center - 1) * (dist_from_center - 1);
    let evens = count_subgrid(&grid, 1, 2) * (dist_from_center) * (dist_from_center);

    let corners = count_subgrid(&grid, 0, 2)
        + count_subgrid(&grid, 2, 0)
        + count_subgrid(&grid, 2, 4)
        + count_subgrid(&grid, 4, 2);

    let lu_edge = count_subgrid(&grid, 0, 1) * dist_from_center
        + count_subgrid(&grid, 1, 1) * (dist_from_center - 1);
    let lb_edge = count_subgrid(&grid, 0, 3) * dist_from_center
        + count_subgrid(&grid, 1, 3) * (dist_from_center - 1);
    let ru_edge = count_subgrid(&grid, 3, 0) * dist_from_center
        + count_subgrid(&grid, 3, 1) * (dist_from_center - 1);
    let rb_edge = count_subgrid(&grid, 3, 4) * dist_from_center
        + count_subgrid(&grid, 3, 3) * (dist_from_center - 1);

    let result = odds + evens + corners + lu_edge + lb_edge + ru_edge + rb_edge;

    // _printgrid(&grid);

    println!("{}", result);

    println!("odds {}", odds);
    println!("evens {}", evens);
    println!("corners {}", corners);
    println!("lu_edge {}", lu_edge);
    println!("lb_edge {}", lb_edge);
    println!("ru_edge {}", ru_edge);
    println!("rb_edge {}", rb_edge);

    // for i in 0..5{
    //     for j in 0..5{
    //         print!("{}", count_subgrid(&grid, i,j))
    //     }
    // }
}
