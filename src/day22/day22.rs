use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

fn _printstack(stack: &Vec<Vec<Vec<usize>>>) {
    for z in (0..stack[0][0].len()).rev() {
        for y in 0..stack[0].len() {
            for x in 0..stack.len() {
                print!("{}", stack[x][y][z])
            }
            println!()
        }
        println!()
    }
}

pub fn run() {
    let blocks: Vec<(Vec<usize>, Vec<usize>)> = include_str!("input")
        .lines()
        .map(|l| {
            l.split("~").map(|splt| {
                splt.split(",")
                    .map(|ustring| ustring.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
        })
        .map(|mut block| (block.next().unwrap(), block.next().unwrap()))
        .collect();

    let xmax = blocks.iter().map(|b| max(b.0[0], b.1[0])).max().unwrap();
    let ymax = blocks.iter().map(|b| max(b.0[1], b.1[1])).max().unwrap();
    let zmax = blocks.iter().map(|b| max(b.0[2], b.1[2])).max().unwrap();

    let mut stack: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; zmax + 1]; ymax + 1]; xmax + 1];

    for x in 0..stack.len() {
        for y in 0..stack[0].len() {
            stack[x][y][0] = 1;
        }
    }

    let mut supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (i, block) in blocks.iter().enumerate() {
        supported_by.insert(i + 2, HashSet::new());

        let block_length = (0..3)
            .map(|j| (block.0[j] as i32 - block.1[j] as i32).abs())
            .max()
            .unwrap() as usize
            + 1;
        let ranges = (0..3)
            .map(|j| {
                if block.0[j] < block.1[j] {
                    (block.0[j]..(block.1[j] + 1)).collect::<Vec<usize>>()
                } else if block.0[j] > block.1[j] {
                    (block.1[j]..(block.0[j] + 1)).rev().collect::<Vec<usize>>()
                } else {
                    vec![block.0[j]; block_length]
                }
            })
            .collect::<Vec<Vec<usize>>>();

        let cubes: Vec<(usize, usize, usize)> = (0..block_length)
            .map(|j| (ranges[0][j], ranges[1][j], ranges[2][j]))
            .collect();

        for cube in cubes {
            stack[cube.0][cube.1][cube.2] = i + 2;
        }
    }

    let mut settled: HashSet<usize> = HashSet::new();
    settled.insert(1);
    while settled.len() < (blocks.len() + 1) {
        let mut changed = true;

        while changed {
            changed = false;

            for z in 1..stack[0][0].len() {
                for x in 0..stack.len() {
                    for y in 0..stack[0].len() {
                        if stack[x][y][z] != 0 {
                            if settled.contains(&stack[x][y][z - 1])
                                && !settled.contains(&stack[x][y][z])
                            {
                                settled.insert(stack[x][y][z]);
                                changed = true;
                            }
                        }
                    }
                }
            }
        }

        for z in 1..stack[0][0].len() {
            for x in 0..stack.len() {
                for y in 0..stack[0].len() {
                    if !settled.contains(&stack[x][y][z]) && stack[x][y][z] != 0 {
                        stack[x][y][z - 1] = stack[x][y][z];
                        stack[x][y][z] = 0;
                    }
                }
            }
        }
    }

    for z in 1..stack[0][0].len() {
        for x in 0..stack.len() {
            for y in 0..stack[0].len() {
                if stack[x][y][z - 1] != 0
                    && stack[x][y][z] != 0
                    && stack[x][y][z - 1] != stack[x][y][z]
                {
                    supported_by
                        .get_mut(&stack[x][y][z])
                        .unwrap()
                        .insert(stack[x][y][z - 1]);
                }
            }
        }
    }

    let mut crucial_supports: HashMap<usize, HashSet<usize>> = HashMap::new();

    for i in 2..(blocks.len() + 2) {
        match supported_by.get(&i) {
            Some(supporting_blocks) => {
                if supporting_blocks.len() == 1 {
                    let supporting_block = *supporting_blocks.iter().next().unwrap();

                    if !crucial_supports.contains_key(&supporting_block) {
                        crucial_supports.insert(supporting_block, HashSet::new());
                    }
                    crucial_supports
                        .get_mut(&supporting_block)
                        .unwrap()
                        .insert(i);
                }
            }
            None => {}
        }
    }
    crucial_supports.remove(&1);
    let part1 = blocks.len() - crucial_supports.len();

    // _printstack(&stack);

    println!("{}", part1);

    fn cascade_block(block: usize, supported_by: &HashMap<usize, HashSet<usize>>) -> usize {
        let mut falling: HashSet<usize> = HashSet::new();
        falling.insert(block);

        let mut changed = true;

        while changed {
            changed = false;
            for (candidate_block, candidate_supports) in supported_by {
                if !falling.contains(candidate_block) {
                    if candidate_supports.iter().all(|s| falling.contains(s)) {
                        falling.insert(*candidate_block);
                        changed = true;
                    }
                }
            }
        }
        return falling.len() - 1;
    }

    let part2: usize = (2..(blocks.len() + 2))
        .map(|b| cascade_block(b, &supported_by))
        .sum();

    println!("part 2 {}", part2)

    // println!("{:?}", supported_by);
}
