#[path = "../utils/astar.rs"]
mod astar;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE,
}

struct Context1 {
    start: State,
    grid: Vec<Vec<u8>>,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
struct State {
    previous: Dir,
    prev_count: usize,
    x: usize,
    y: usize,
}

impl astar::Astarcontext<State> for Context1 {
    fn start(&self) -> State {
        self.start
    }

    fn neighbours(&self, state: &State) -> Vec<(State, usize)> {
        let mut result: Vec<(State, usize)> = vec![];
        for (i, j, next_dir) in [
            (-1, 0, Dir::UP),
            (1, 0, Dir::DOWN),
            (0, -1, Dir::LEFT),
            (0, 1, Dir::RIGHT),
        ] {
            if (state.y as i32 + i) >= 0
                && (state.y as i32 + i) < self.grid.len() as i32
                && (state.x as i32 + j) >= 0
                && (state.x as i32 + j) < self.grid[0].len() as i32
                && !(state.prev_count >= 3 && state.previous == next_dir)
                && match (state.previous, next_dir) {
                    (Dir::LEFT, Dir::RIGHT) => false,
                    (Dir::RIGHT, Dir::LEFT) => false,
                    (Dir::UP, Dir::DOWN) => false,
                    (Dir::DOWN, Dir::UP) => false,
                    _ => true

                }
                
            {
                result.push((
                    State {
                        previous: next_dir,
                        prev_count: if next_dir == state.previous {state.prev_count + 1} else {1},
                        y: (state.y as i32 + i) as usize,
                        x: (state.x as i32 + j) as usize,
                    },
                    self.grid[(state.y as i32 + i) as usize][(state.x as i32 + j) as usize]
                        as usize,
                ));
            }
        }
        return result;
    }

    fn heuristic(&self, state: &State) -> usize {
        let h = self.grid.len() - state.y - 1 + self.grid[0].len() - state.x - 1;
        return h;
        // return 0
    }

    fn goal(&self, state: &State) -> bool {
        state.x == (self.grid[0].len() - 1) && state.y == (self.grid.len() - 1)
    }
}

struct Context2 {
    start: State,
    grid: Vec<Vec<u8>>,
}


impl astar::Astarcontext<State> for Context2 {
    fn start(&self) -> State {
        self.start
    }

    fn neighbours(&self, state: &State) -> Vec<(State, usize)> {
        let mut result: Vec<(State, usize)> = vec![];
        for (i, j, next_dir) in [
            (-1, 0, Dir::UP),
            (1, 0, Dir::DOWN),
            (0, -1, Dir::LEFT),
            (0, 1, Dir::RIGHT),
        ] {
            if (state.y as i32 + i) >= 0
                && (state.y as i32 + i) < self.grid.len() as i32
                && (state.x as i32 + j) >= 0
                && (state.x as i32 + j) < self.grid[0].len() as i32
                && !(state.prev_count >= 10 && state.previous == next_dir)
                && (state.prev_count >= 4 || state.previous == next_dir || state.previous == Dir::NONE)
                && match (state.previous, next_dir) {
                    (Dir::LEFT, Dir::RIGHT) => false,
                    (Dir::RIGHT, Dir::LEFT) => false,
                    (Dir::UP, Dir::DOWN) => false,
                    (Dir::DOWN, Dir::UP) => false,
                    _ => true

                }
                
            {
                result.push((
                    State {
                        previous: next_dir,
                        prev_count: if next_dir == state.previous {state.prev_count + 1} else {1},
                        y: (state.y as i32 + i) as usize,
                        x: (state.x as i32 + j) as usize,
                    },
                    self.grid[(state.y as i32 + i) as usize][(state.x as i32 + j) as usize]
                        as usize,
                ));
            }
        }
        return result;
    }

    fn heuristic(&self, state: &State) -> usize {
        let h = self.grid.len() - state.y - 1 + self.grid[0].len() - state.x - 1;
        return h;
        // return 0
    }

    fn goal(&self, state: &State) -> bool {
        state.x == (self.grid[0].len() - 1) && state.y == (self.grid.len() - 1) && state.prev_count >= 4
    }
}

pub fn run() {

    let grid: Vec<Vec<u8>> = include_str!("input")
    .lines()
    .map(|l| {
        l.chars()
            .map(|chr| chr.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>()
    })
    .collect();

    let context = Context1 {
        start: State {
            previous: Dir::NONE,
            prev_count: 0,
            x: 0,
            y: 0,
        },
        grid: grid.clone(),
    };

    let (heatloss, _) = astar::a_star(&context);

    println!("part 1 {}", heatloss);

    let context = Context2 {
        start: State {
            previous: Dir::NONE,
            prev_count: 0,
            x: 0,
            y: 0,
        },
        grid: grid.clone(),
    };

    let (heatloss, _) = astar::a_star(&context);

    println!("part 2 {}", heatloss);




}
