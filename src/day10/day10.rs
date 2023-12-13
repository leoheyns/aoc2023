use std::collections::HashSet;
use std::vec;

fn neighbours(coord: (usize, usize)) -> Vec<(usize, usize)> {
    vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        .map(|offset| {
            (
                (offset.0 + (coord.0 as i32)) as usize,
                (offset.1 + (coord.1 as i32)) as usize,
            )
        })
        .collect()
}

fn pipe_neighbours(coord: (usize, usize), pipe: char) -> Vec<(usize, usize)> {
    match pipe {
        '-' => vec![(coord.0, coord.1 - 1), (coord.0, coord.1 + 1)],
        '7' => vec![(coord.0, coord.1 - 1), (coord.0 + 1, coord.1)],
        '|' => vec![(coord.0 - 1, coord.1), (coord.0 + 1, coord.1)],
        'J' => vec![(coord.0, coord.1 - 1), (coord.0 - 1, coord.1)],
        'L' => vec![(coord.0 - 1, coord.1), (coord.0, coord.1 + 1)],
        'F' => vec![(coord.0 + 1, coord.1), (coord.0, coord.1 + 1)],
        _ => vec![(0, 0), (0, 0)],
    }
}

fn pipe_edges(
    from: (usize, usize),
    to: (usize, usize),
    pipe: char,
    left: bool,
) -> Vec<(usize, usize)> {
    match pipe {
        '-' => {
            if !((from.1 == (to.1 - 1)) ^ left) {
                vec![(to.0 - 1, to.1)]
            } else {
                vec![(to.0 + 1, to.1)]
            }
        }
        '7' => {
            if (from.0 == (to.0 + 1)) ^ left {
                vec![(to.0 - 1, to.1), (to.0, to.1 + 1)]
            } else {
                vec![]
            }
        }
        '|' => {
            if !((from.0 == (to.0 - 1)) ^ left) {
                vec![(to.0, to.1 + 1)]
            } else {
                vec![(to.0, to.1 - 1)]
            }
        }
        'J' => {
            if (from.1 == (to.1 - 1)) ^ left {
                vec![(to.0, to.1 + 1), (to.0 + 1, to.1)]
            } else {
                vec![]
            }
        }
        'L' => {
            if (from.0 == (to.0 - 1)) ^ left {
                vec![(to.0, to.1 - 1), (to.0 + 1, to.1)]
            } else {
                vec![]
            }
        }
        'F' => {
            if (from.1 == (to.1 + 1)) ^ left {
                vec![(to.0 - 1, to.1), (to.0, to.1 - 1)]
            } else {
                vec![]
            }
        }
        _ => vec![],
    }
}

fn get_lefts(from: (usize, usize), to: (usize, usize), pipe: char) -> i32 {
    match pipe {
        '-' => 0,
        '7' => {
            if from.0 == (to.0 + 1) {
                1
            } else {
                -1
            }
        }
        '|' => 0,
        'J' => {
            if from.1 == (to.1 - 1) {
                1
            } else {
                -1
            }
        }
        'L' => {
            if from.0 == (to.0 - 1) {
                1
            } else {
                -1
            }
        }
        'F' => {
            if from.1 == (to.1 + 1) {
                1
            } else {
                -1
            }
        }
        _ => 0,
    }
}

pub fn run() {
    let mut grid: Vec<Vec<char>> = include_str!("testinput")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    grid.insert(0, vec!['.'; grid[0].len()]);
    grid.push(vec!['.'; grid[0].len()]);

    for line in &mut grid {
        line.insert(0, '.');
        line.push('.');
    }

    let mut start_coord = (0, 0);

    for (i, line) in grid.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'S' {
                start_coord = (i, j)
            }
        }
    }

    let mut path: Vec<(usize, usize)> = vec![]; //y, x, distance

    let mut path_length = 0;

    path.push(start_coord);

    for (i, neighbour) in neighbours(start_coord).iter().enumerate() {
        //north
        if i == 0 && vec!['F', '|', '7'].contains(&grid[neighbour.0][neighbour.1]) {
            path.push(*neighbour);
            break;
        }
        if i == 1 && vec!['J', '7', '-'].contains(&grid[neighbour.0][neighbour.1]) {
            path.push(*neighbour);
            break;
        }
        if i == 2 && vec!['J', '|', 'L'].contains(&grid[neighbour.0][neighbour.1]) {
            path.push(*neighbour);
            break;
        }
        if i == 3 && vec!['L', '-', 'F'].contains(&grid[neighbour.0][neighbour.1]) {
            path.push(*neighbour);
        }
    }

    while path[path.len() - 1] != start_coord {
        let current = path[path.len() - 1];
        let previous = path[path.len() - 2];

        path.push(
            *pipe_neighbours(current, grid[current.0][current.1])
                .iter()
                .filter(|coord| **coord != previous)
                .next()
                .unwrap(),
        );
        path_length += 1;
    }

    //part 1
    println!("{}", path_length / 2);

    let lefts: i32 = path
        .windows(2)
        .map(|w| get_lefts(w[0], w[1], grid[w[1].0][w[1].1]))
        .sum();

    let inner_node_init: Vec<(usize, usize)> = path
        .windows(2)
        .flat_map(|w| pipe_edges(w[0], w[1], grid[w[1].0][w[1].1], lefts > 0))
        .filter(|coord| !path.contains(coord))
        .collect();

    // println!("{}", lefts);
    // println!("{:?}", inner_node_init);

    let mut print_grid = vec![vec![' '; grid[0].len()]; grid.len()];

    for coord in path.clone() {
        print_grid[coord.0][coord.1] = match grid[coord.0][coord.1] {
            'J' => '┘',
            'F' => '┌',
            '7' => '┐',
            'L' => '└',
            '-' => '─',
            '|' => '│',
            _ => ' ',
        };
    }

    // for coord in inner_node_init.clone() {
    //     print_grid[coord.0][coord.1] = '*';
    // }

    //└─┐│┌┘

    // let fill_grid = print_grid.clone();

    let mut queue = inner_node_init.clone();
    let mut visited_fill: HashSet<(usize, usize)> =
        HashSet::from_iter(inner_node_init.iter().map(|v| *v));

    let path_set: HashSet<(usize, usize)> = HashSet::from_iter(path.iter().map(|v| *v));

    while queue.len() > 0 {
        let current = queue.pop().unwrap();
        for neighbour in neighbours(current) {
            if !path_set.contains(&neighbour) && !visited_fill.contains(&neighbour) {
                queue.push(neighbour);
                visited_fill.insert(neighbour);
            }
        }
    }

    println!("{}", visited_fill.len());

    for coord in visited_fill.clone() {
        print_grid[coord.0][coord.1] = '*';
    }

    for line in print_grid.iter() {
        for str in line {
            print!("{}", str);
        }
        println!();
    }
}
