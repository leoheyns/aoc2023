use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn get_energized(grid: &Vec<Vec<char>>, entry: ((usize, usize), Direction)) -> usize {
    let mut beam_heads: Vec<((usize, usize), Direction)> =
        match (entry.1, grid[entry.0 .0][entry.0 .1]) {
            (Direction::RIGHT | Direction::LEFT, '|') => {
                vec![(entry.0, Direction::UP), (entry.0, Direction::DOWN)]
            }
            (Direction::UP | Direction::DOWN, '-') => {
                vec![(entry.0, Direction::LEFT), (entry.0, Direction::RIGHT)]
            }
            (Direction::UP, '/') => vec![(entry.0, Direction::RIGHT)],
            (Direction::DOWN, '/') => vec![(entry.0, Direction::LEFT)],
            (Direction::LEFT, '/') => vec![(entry.0, Direction::DOWN)],
            (Direction::RIGHT, '/') => vec![(entry.0, Direction::UP)],
            (Direction::UP, '\\') => vec![(entry.0, Direction::LEFT)],
            (Direction::DOWN, '\\') => vec![(entry.0, Direction::RIGHT)],
            (Direction::LEFT, '\\') => vec![(entry.0, Direction::UP)],
            (Direction::RIGHT, '\\') => vec![(entry.0, Direction::DOWN)],
            _ => vec![entry],
        };

    let mut beams: HashSet<((usize, usize), Direction)> = HashSet::new();

    for beam in beam_heads.clone() {
        beams.insert(beam);
    }

    while beam_heads.len() > 0 {
        let head = beam_heads.pop().unwrap();
        if match head.1 {
            Direction::RIGHT => head.0 .1 < (grid[0].len() - 1),
            Direction::LEFT => head.0 .1 > 0,
            Direction::UP => head.0 .0 > 0,
            Direction::DOWN => head.0 .0 < (grid.len() - 1),
        } {
            let next_coord = match head.1 {
                Direction::RIGHT => (head.0 .0, head.0 .1 + 1),
                Direction::LEFT => (head.0 .0, head.0 .1 - 1),
                Direction::UP => (head.0 .0 - 1, head.0 .1),
                Direction::DOWN => (head.0 .0 + 1, head.0 .1),
            };

            let next_heads = match (head.1, grid[next_coord.0][next_coord.1]) {
                (Direction::RIGHT | Direction::LEFT, '|') => {
                    vec![(next_coord, Direction::UP), (next_coord, Direction::DOWN)]
                }
                (Direction::UP | Direction::DOWN, '-') => vec![
                    (next_coord, Direction::LEFT),
                    (next_coord, Direction::RIGHT),
                ],
                (Direction::UP, '/') => vec![(next_coord, Direction::RIGHT)],
                (Direction::DOWN, '/') => vec![(next_coord, Direction::LEFT)],
                (Direction::LEFT, '/') => vec![(next_coord, Direction::DOWN)],
                (Direction::RIGHT, '/') => vec![(next_coord, Direction::UP)],
                (Direction::UP, '\\') => vec![(next_coord, Direction::LEFT)],
                (Direction::DOWN, '\\') => vec![(next_coord, Direction::RIGHT)],
                (Direction::LEFT, '\\') => vec![(next_coord, Direction::UP)],
                (Direction::RIGHT, '\\') => vec![(next_coord, Direction::DOWN)],
                _ => vec![(next_coord, head.1)],
            };
            // println!("{:?} {:?}", head, next_heads);

            for next_head in next_heads {
                if !beams.contains(&next_head) {
                    beams.insert(next_head);
                    beam_heads.push(next_head);
                }
            }
        }
    }

    // println!("{:?}", beams);

    let energized_tiles: HashSet<(usize, usize)> = HashSet::from_iter(beams.iter().map(|b| b.0));

    return energized_tiles.len();
}

pub fn run() {
    let grid = include_str!("input")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let entry = ((0, 0), Direction::RIGHT);

    let part1 = get_energized(&grid, entry);
    println!("{}", part1);

    let entries = (0..grid.len())
        .map(|i| ((i, 0), Direction::RIGHT))
        .chain((0..grid.len()).map(|i| ((i, grid[0].len() - 1), Direction::LEFT)))
        .chain((0..grid[0].len()).map(|i| ((0, i), Direction::DOWN)))
        .chain((0..grid.len()).map(|i| ((grid.len() - 1, i), Direction::UP)));

    let energies = entries.map(|e| get_energized(&grid, e));

    let part2 = energies.max().unwrap();
    println!("{}", part2);
}
