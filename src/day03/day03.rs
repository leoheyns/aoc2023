use std::usize;

pub fn run(){
    let lines: Vec<Vec<char>> = include_str!("input").lines().map(|l| l.chars().collect()).collect();
    let mut part1 = 0;
    let mut is_part_num = false;
    let mut current_num = "".to_owned();
    for i in 0..lines.len(){

        if is_part_num && current_num.len() > 0{
            part1 += current_num.parse::<usize>().unwrap();
        }

        is_part_num = false;
        current_num = "".to_owned();


        for j in 0..lines[0].len(){
            if !lines[i][j].is_digit(10){
                if is_part_num && current_num.len() > 0{
                    part1 += current_num.parse::<usize>().unwrap();
                }
                current_num = "".to_owned();
                is_part_num = false;
            } else {
                current_num.push_str(lines[i][j].to_string().as_str());

            }

            for k in 0..=2{
                for l in 0..=2{
                    let check_y = (i + k) as i32 - 1;
                    let check_x = (j + l) as i32 - 1;


                    if check_y < (lines.len() as i32) && check_y >= 0 && check_x < lines[0].len() as i32 && check_x >= 0 && lines[i][j].is_digit(10){
                        if lines[i+k-1][j+l-1] != '.' && !lines[i+k-1][j+l-1].is_digit(10){
                            is_part_num = true;
                        }
                    }                    
                }
            }

        }
    }

    println!("{}", part1);




    let lines: Vec<Vec<char>> = include_str!("input").lines().map(|l| l.chars().collect()).collect();
    let mut part2 = 0;
    let mut is_part_num = false;
    let mut current_num = "".to_owned();
    let mut num_grid = vec![vec![(0, 0); lines[0].len()]; lines.len()];
    let mut num_count = 0;

    for i in 0..lines.len(){

        if is_part_num && current_num.len() > 0{
            num_count += 1;
            for j2 in 0..current_num.len(){
                num_grid[i - 1][lines[0].len() - 1 - j2] = (num_count, current_num.parse::<usize>().unwrap())
            }
        }

        is_part_num = false;
        current_num = "".to_owned();


        for j in 0..lines[0].len(){
            if !lines[i][j].is_digit(10){
                if is_part_num && current_num.len() > 0{
                    // println!("{}", current_num);
                    // part1 += current_num.parse::<usize>().unwrap();
                    num_count += 1;
                    for j2 in 0..current_num.len(){
                        num_grid[i][j-j2 - 1] = (num_count, current_num.parse::<usize>().unwrap())
                    }
                }
                current_num = "".to_owned();
                is_part_num = false;
            } else {
                current_num.push_str(lines[i][j].to_string().as_str());

            }

            for k in 0..=2{
                for l in 0..=2{
                    let check_y = (i + k) as i32 - 1;
                    let check_x = (j + l) as i32 - 1;


                    if check_y < (lines.len() as i32) && check_y >= 0 && check_x < lines[0].len() as i32 && check_x >= 0 && lines[i][j].is_digit(10){
                        if lines[i+k-1][j+l-1] == '*'{
                            is_part_num = true;
                        }
                    }                    
                }
            }

        }
    }

    println!("{}", num_count);

    for i in 0..lines.len(){
        for j in 0..lines[0].len(){
            if lines[i][j] == '*'{
                let mut num_1 = (0,0);
                let mut num_2 = (0,0);
                for k in 0..=2{
                    for l in 0..=2{
                        let check_y = (i + k) as i32 - 1;
                        let check_x = (j + l) as i32 - 1;
    
    
                        if check_y < (lines.len() as i32) && check_y >= 0 && check_x < lines[0].len() as i32 && check_x >= 0{
                            if num_grid[i+k-1][j+l-1].0 > 0{
                                if num_1 == (0,0){
                                    num_1 = num_grid[i+k-1][j+l-1]
                                } else if num_2 == (0,0) && num_1.0 != num_grid[i+k-1][j+l-1].0{
                                    num_2 = num_grid[i+k-1][j+l-1]
                                }
                            }
                        }                    
                    }
                }

                part2 += num_1.1 * num_2.1;
            }
        }
    }
    println!("{} {}", num_grid[2][8].0, num_grid[2][8].1);
    println!("{}", part2);

}
