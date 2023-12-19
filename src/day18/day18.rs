use std::{collections::HashSet, vec};

fn split_line(
    cline: ((i64, i64), (i64, i64)),
    nline: ((i64, i64), (i64, i64)),
) -> Vec<((i64, i64), (i64, i64))> {
    let mut result: Vec<((i64, i64), (i64, i64))> = vec![];
    if nline.0 .1 > cline.0 .1 {
        result.push(((nline.0 .0, cline.0 .1), (nline.0 .0, nline.0 .1)));
    }
    if nline.1 .1 < cline.1 .1 {
        result.push(((nline.0 .0, nline.1 .1), (nline.0 .0, cline.1 .1)));
    }

    return result;
}

fn merge_lines(lines: Vec<((i64, i64), (i64, i64))>) -> Vec<((i64, i64), (i64, i64))> {
    let mut result: Vec<((i64, i64), (i64, i64))> = vec![];
    if lines.len() == 0 {
        return result;
    }
    let mut current = lines[0];
    for i in 1..lines.len() {
        if current.1 .1 == lines[i].0 .1 {
            current = ((current.0 .0, current.0 .1), (current.0 .0, lines[i].1 .1))
        } else {
            result.push(current);
            current = lines[i];
        }
    }
    result.push(current);
    return result;
}

pub fn run() {
    let input = include_str!("input")
        .lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>());

    let mut holes: HashSet<(i64, i64)> = HashSet::new();
    let mut current_hole = (0, 0);
    holes.insert(current_hole);

    for instruction in input {
        let dir = match instruction[0] {
            "U" => (-1, 0),
            "D" => (1, 0),
            "L" => (0, -1),
            "R" => (0, 1),
            _ => (0, 0),
        };

        for _ in 0..instruction[1].parse::<usize>().unwrap() {
            current_hole = (current_hole.0 + dir.0, current_hole.1 + dir.1);
            holes.insert(current_hole);
        }
    }

    let corner = holes.iter().max().unwrap();

    let mut frontier = vec![(corner.0 - 1, corner.1 - 1)];

    while frontier.len() > 0 {
        let current = frontier.pop().unwrap();
        for (yoffset, xoffset) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let new_hole = (current.0 + yoffset, current.1 + xoffset);
            if !holes.contains(&new_hole) {
                frontier.push(new_hole);
                holes.insert(new_hole);
            }
        }
    }

    println!("part 1 {}", holes.len());

    let binding = include_str!("input").replace(")", "");
    let input = binding.lines().map(|l| l.split("(#").last().unwrap());

    let mut lines: Vec<((i64, i64), (i64, i64))> = vec![];
    let mut current_hole = (0, 0);
    holes.insert(current_hole);

    for instruction in input {
        let inst_dir = instruction.chars().last().unwrap();
        let dir: (i64, i64) = match inst_dir {
            '0' => (0, 1),
            '1' => (1, 0),
            '2' => (0, -1),
            '3' => (-1, 0),
            _ => (0, 0),
        };
        let distance =
            usize::from_str_radix(&instruction[..instruction.len() - 1], 16).unwrap() as i64;

        let new_hole = (
            current_hole.0 + dir.0 * distance,
            current_hole.1 + dir.1 * distance,
        );

        if inst_dir == '0' {
            lines.push((current_hole, new_hole));
        } else if inst_dir == '2' {
            lines.push((new_hole, current_hole));
        }

        current_hole = new_hole;
    }
    lines.sort();

    let mut result = 0;
    let mut current_lines = vec![lines.pop().unwrap()];

    while lines[lines.len() - 1].0 .0 == current_lines[0].0 .0 {
        current_lines.push(lines.pop().unwrap());
    }

    while lines.len() > 0 {
        let mut next_lines = vec![lines.pop().unwrap()];

        if lines.len() > 0 {
            while lines[lines.len() - 1].0 .0 == next_lines[0].0 .0 {
                next_lines.push(lines.pop().unwrap());
            }
        }

        // println!("current {:?}", current_lines);
        // println!("next {:?}", next_lines);

        for line in &current_lines {
            result += (line.1 .1 - line.0 .1 + 1) * (line.0 .0 - next_lines[0].0 .0)
        }

        let mut next_current_lines: Vec<((i64, i64), (i64, i64))> = vec![];
        let mut cut_lines: HashSet<((i64, i64), (i64, i64))> = HashSet::new();
        let mut line_queue = current_lines.clone();
        while line_queue.len() > 0 {
            let cline = line_queue.pop().unwrap();
            let mut handled = false;
            for nline in &next_lines {
                if nline.0 .1 == cline.0 .1 && nline.1 .1 == cline.1 .1 {
                    handled = true;
                    cut_lines.insert(*nline);
                } else if nline.0 .1 >= cline.0 .1 && nline.1 .1 <= cline.1 .1 {
                    handled = true;
                    for splt_line in split_line(cline, *nline) {
                        line_queue.push(splt_line);
                    }
                    cut_lines.insert(*nline);
                    break;
                }
            }
            if !handled {
                next_current_lines.push((
                    (next_lines[0].0 .0, cline.0 .1),
                    (next_lines[0].0 .0, cline.1 .1),
                ));
            }
        }

        for l in next_lines.iter().filter(|line| !cut_lines.contains(line)) {
            next_current_lines.push(*l);
        }

        for line in cut_lines {
            let mut terminal_squares = line.1 .1 - line.0 .1 + 1; //number of squares with nothing above it.
            for corner in [line.0, line.1] {
                if next_current_lines
                    .clone()
                    .iter()
                    .any(|nl| nl.0 == corner || nl.1 == corner)
                {
                    terminal_squares -= 1;
                }
            }

            result += terminal_squares;
        }

        next_current_lines.sort();

        current_lines = merge_lines(next_current_lines);
    }

    println!("part 2 {}", result);
}
