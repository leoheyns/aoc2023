use std::cmp::min;

fn getnum(chars: std::str::Chars<'_>) -> u32{
    let cs = chars.collect::<Vec<char>>();
    let mut found = false;
    let mut firstnum: u32 = 0;
    let mut lastnum: u32 = 0;
    for i in 0..(cs.len()){
        println!("{}", cs[i]);
        if !found && cs[i].is_digit(10){
            found = true;
            firstnum = cs[i].to_digit(10).unwrap();
        }

        if cs[i].is_digit(10){
            lastnum = cs[i].to_digit(10).unwrap();
        }

        for (num, word) in vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter().enumerate(){
            if cs[i..min(i+word.len(), cs.len())] == word.chars().collect::<Vec<char>>(){
                if !found {
                    found = true;
                    firstnum = num as u32 + 1;
                }
                    lastnum = num as u32 + 1;
            }
                   
        } 
    }

    return firstnum * 10 + lastnum
}

pub fn run(){
    let input: u32 = include_str!("input")

    .lines().map(|l| l.chars())
    .map(|cs: std::str::Chars<'_>| getnum(cs)).sum();

    println!("{}", input);
}
