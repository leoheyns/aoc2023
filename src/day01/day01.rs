fn getnum(chars: std::str::Chars<'_>) -> u32 {
    let mut found = false;
    let mut firstnum: char = '0';
    let mut lastnum: char = '0';
    for c in chars {
        if !found && c.is_digit(10) {
            found = true;
            firstnum = c;
        }

        if c.is_digit(10) {
            lastnum = c;
        }
    }

    return firstnum.to_digit(10).unwrap() * 10 + lastnum.to_digit(10).unwrap();
}

pub fn run() {
    let input: u32 = include_str!("input")
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eitht8eight")
        .replace("nine", "nine9nine")
        .lines()
        .map(|l| l.chars())
        .map(|cs: std::str::Chars<'_>| getnum(cs))
        .sum();

    println!("{}", input);
}
