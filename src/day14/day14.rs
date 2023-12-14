fn tilt(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = vec![vec!['.'; grid[0].len()]; grid.len()];
    for i in 0..grid[0].len() {
        let mut stop = 0;
        for j in 0..grid.len() {
            if grid[j][i] == '#' {
                result[j][i] = '#';
                stop = j + 1;
            } else if grid[j][i] == 'O' {
                result[stop][i] = 'O';
                if stop != j {
                    result[j][i] = '.';
                }
                stop += 1;
            } else {
                result[j][i] = '.';
            }
        }
    }
    return result;
}

fn load(grid: Vec<Vec<char>>) -> usize {
    let mut result = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                result += grid.len() - i
            }
        }
    }
    return result;
}

fn rotate(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rotated = vec![vec!['.'; grid.len()]; grid[0].len()];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            rotated[j][grid.len() - i - 1] = grid[i][j]
        }
    }

    return rotated;
}

fn cycle(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut current_grid = grid.clone();
    for _ in 0..4 {
        current_grid = rotate(tilt(current_grid));
    }
    return current_grid;
}

pub fn run() {
    let input = include_str!("input")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut before;
    let mut after = input;

    let mut past_loads: Vec<usize> = vec![];

    for i in 0..1000000000 {
        before = after;
        after = cycle(before.clone());
        let new_load = load(after.clone());
        past_loads.push(new_load);
        let mut stop = false;
        if past_loads.len() > 10 {
            for j in 5..(past_loads.len() / 2) {
                if past_loads[past_loads.len() - j - 1..].to_vec()
                    == past_loads[past_loads.len() - 2 * j - 1..past_loads.len() - j].to_vec()
                {
                    stop = true;
                    println!("{:?}", past_loads);
                    println!(
                        "{}",
                        past_loads[(past_loads.len() - j - 1) + ((1000000000 - i - 1) % j)]
                    );
                    break;
                }
            }
        }
        if stop {
            break;
        }
    }
}
