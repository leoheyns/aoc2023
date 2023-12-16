fn hash_string(inp: &str) -> u8 {
    inp.bytes()
        .fold(0, |current, b| (current.wrapping_add(b)).wrapping_mul(17))
}

pub fn run() {
    let binding = include_str!("input").replace("\n", "").replace("-", "--");
    let input = binding.split(",").collect::<Vec<&str>>();
    let mut boxes: Vec<Vec<(&str, u8)>> = vec![vec![]; 256];

    for instruction in input {
        let label = &instruction[..instruction.len() - 2];
        let hash = hash_string(label) as usize;
        let index = boxes[hash].iter().position(|(l, _)| l == &label);

        if instruction.chars().last().unwrap() == '-' {
            match index {
                Some(i) => {
                    boxes[hash].remove(i);
                }
                None => {}
            }
        } else {
            let val = instruction.chars().last().unwrap().to_digit(10).unwrap() as u8;
            match index {
                Some(i) => {
                    boxes[hash][i] = (label, val);
                }
                None => boxes[hash].push((label, val)),
            }
        }
    }

    let mut part2 = 0;
    for i in 0..boxes.len() {
        for j in 0..boxes[i].len() {
            part2 += (i + 1) * (j + 1) * (boxes[i][j].1 as usize)
        }
    }

    println!("{}", part2)
}
