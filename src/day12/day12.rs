use cached::proc_macro::cached;

#[cached]
fn arrangements(springs: String, groups: Vec<usize>) -> usize {
    if groups[0] > springs.len() {
        return 0;
    }
    let mut result = 0;
    let spring_chars = springs.chars().collect::<Vec<char>>();
    for i in 0..(springs.len() - groups[0] + 1) {
        if i > 0 {
            if spring_chars[i - 1] == '#' {
                break;
            }
        }

        if springs[i..(i + groups[0])]
            .chars()
            .all(|c| c == '#' || c == '?')
        {
            if groups.len() == 1 {
                if springs[i + groups[0]..]
                    .chars()
                    .all(|c| c == '?' || c == '.')
                {
                    result += 1
                }
            } else if springs.len() > i + groups[0] {
                if spring_chars[i + groups[0]] != '#' {
                    result += arrangements(springs[(i + groups[0] + 1)..].to_string(), groups[1..].to_vec())
                }
            }
        }
    }

    return result;
}

pub fn run() {
    let input = include_str!("input")
        .lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .map(|splt| {
            (
                splt[0],
                splt[1]
                    .split(",")
                    .map(|numstring| numstring.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        });

    let results = input
        .map(|l| arrangements(l.0.to_string(), l.1))
        .collect::<Vec<usize>>();

        let input2 = include_str!("input")
        .lines()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .map(|splt| {
            (
                vec![splt[0];5].join("?"),
                std::iter::repeat(splt[1]
                    .split(",")
                    .map(|numstring| numstring.parse::<usize>().unwrap())
                ).take(5).flatten().collect::<Vec<usize>>()
            )

        });

    let results2 = input2
        .map(|l| arrangements(l.0.to_string(), l.1))
        .collect::<Vec<usize>>();
    // let input2 = include_str!("testinput")
    //     .lines()
    //     .map(|l| l.split(" ").collect::<Vec<&str>>())
    //     .map(|splt| {
    //         (
    //             vec![splt[0];5].join("?"),
    //             std::iter::repeat(splt[1]
    //                 .split(",")
    //                 .map(|numstring| numstring.parse::<usize>().unwrap())
    //             ).take(5).flatten().collect::<Vec<usize>>()
    //         )

    //     });

    // for (res, inp) in std::iter::zip(results2.iter(), input2){
    //     if *res == 35{
    //         println!("{:?} {}", inp, res)
    //     }
    // }

    // for res in results2{
    //     println!("{:?}", res)
    // }
    println!("{}", results.iter().sum::<usize>());
    println!("{}", results2.iter().sum::<usize>())

}
