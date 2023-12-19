use std::{collections::HashMap, usize};

#[derive(PartialEq, Eq, Debug)]
enum BoolOp {
    LT,
    GT,
}

#[derive(PartialEq, Eq, Debug)]
enum Instr<'a> {
    COND(u64, BoolOp, u64, &'a str),
    GOTO(&'a str),
}

fn interpret_instr(inst_str: &str) -> Instr {
    if inst_str.split(":").count() > 1 {
        //we have a cond
        let mut splt = inst_str.split(":");
        let cond = splt.next().unwrap();
        let dest = splt.next().unwrap();

        return Instr::COND(
            match cond.as_bytes()[0] as char {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => 100,
            },
            if cond.as_bytes()[1] as char == '>' {
                BoolOp::GT
            } else {
                BoolOp::LT
            },
            cond[2..].parse::<u64>().unwrap(),
            dest,
        );
    } else {
        return Instr::GOTO(inst_str);
    }
}

pub fn run() {
    let mut input = include_str!("testinput").split("\n\n");
    let rules: Vec<(&str, Vec<Instr<'_>>)> = input
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split("{").collect::<Vec<&str>>())
        .map(|splt| {
            (
                splt[0],
                splt[1][..splt[1].len() - 1]
                    .split(",")
                    .map(interpret_instr)
                    .collect::<Vec<Instr>>(),
            )
        })
        .collect();

    let parts = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l[1..l.len() - 1]
                .split(",")
                .map(|val| val[2..].parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let mut rules_dict: HashMap<&str, Vec<Instr<'_>>> = HashMap::new();

    for (key, value) in rules {
        rules_dict.insert(key, value);
    }

    fn process_part(
        part: &Vec<u64>,
        workflow: &str,
        rules_dict: &HashMap<&str, Vec<Instr<'_>>>,
    ) -> bool {
        if workflow == "A" {
            return true;
        } else if workflow == "R" {
            return false;
        }
        for instr in rules_dict.get(workflow).unwrap() {
            match instr {
                Instr::COND(val_index, BoolOp::GT, comp_value, dest) => {
                    if part[*val_index as usize] > *comp_value {
                        return process_part(part, dest, rules_dict);
                    }
                }
                Instr::COND(val_index, BoolOp::LT, comp_value, dest) => {
                    if part[*val_index as usize] < *comp_value {
                        return process_part(part, dest, rules_dict);
                    }
                }
                Instr::GOTO(dest) => return process_part(part, dest, rules_dict),
            }
        }
        return false;
    }

    let results = parts
        .iter()
        .filter(|part| process_part(part, "in", &rules_dict))
        .map(|p| p.iter().sum::<u64>())
        .sum::<u64>();

    println!("part 1{}", results);

    fn process_range(
        part_range: Vec<(u64, u64)>,
        workflow: &str,
        rules_dict: &HashMap<&str, Vec<Instr<'_>>>,
    ) -> u64 {
        println!("workflow: {}", workflow);
        println!("parts {:?}", part_range);
        println!();

        if workflow == "A" {
            let valid_part_count = part_range
                .iter()
                .fold(1, |current, range| current * (range.1 - range.0));
            return valid_part_count;
        } else if workflow == "R" {
            return 0;
        }

        return rules_dict
            .get(workflow)
            .unwrap()
            .iter()
            .fold((part_range.clone(), 0), |(pr, result), instr| match instr {
                Instr::COND(val_index, BoolOp::GT, comp_value, dest) => {
                    if pr[*val_index as usize].1 > (*comp_value + 1) {
                        let mut true_range = part_range.clone();
                        let mut false_range = part_range.clone();

                        true_range[*val_index as usize].0 =
                            std::cmp::max(*comp_value + 1, true_range[*val_index as usize].0);
                        false_range[*val_index as usize].1 = std::cmp::max(
                            std::cmp::min(*comp_value + 1, false_range[*val_index as usize].1),
                            false_range[*val_index as usize].0,
                        );

                        println!("instruction {:?}", instr);
                        println!("camefrom {}", workflow);
                        println!("false_range: {:?}", false_range);
                        (
                            false_range,
                            result + process_range(true_range, dest, rules_dict),
                        )
                    } else {
                        (pr, result)
                    }
                }
                Instr::COND(val_index, BoolOp::LT, comp_value, dest) => {
                    if pr[*val_index as usize].0 < *comp_value {
                        let mut true_range = part_range.clone();
                        let mut false_range = part_range.clone();

                        true_range[*val_index as usize].1 =
                            std::cmp::min(*comp_value, true_range[*val_index as usize].1);
                        false_range[*val_index as usize].0 = std::cmp::min(
                            std::cmp::max(*comp_value, false_range[*val_index as usize].0),
                            false_range[*val_index as usize].1,
                        );

                        println!("camefrom {}", workflow);
                        println!("instruction {:?}", instr);
                        println!("false_range: {:?}", false_range);
                        (
                            false_range,
                            result + process_range(true_range, dest, rules_dict),
                        )
                    } else {
                        (pr, result)
                    }
                }
                Instr::GOTO(dest) => {
                    println!("instruction {:?}", instr);
                    println!("camefrom {}", workflow);
                    (
                        vec![(0 as u64, 0 as u64); 4],
                        result + process_range(pr, dest, rules_dict),
                    )
                }
            })
            .1;
    }

    let parts_range: Vec<(u64, u64)> = vec![(1, 4001), (1, 4001), (1, 4001), (1, 4001)];
    let part2 = process_range(parts_range, "in", &rules_dict);

    println!("part 2 {}", part2);
}
