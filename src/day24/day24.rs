use std::ops::Add;

#[derive(Debug)]
struct Coord {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct Hailstone {
    pos: Coord,
    speed: Coord,
}

fn hail_collides(stone_1: &Hailstone, stone_2: &Hailstone, bounds: (f64, f64)) -> bool {
    let numerator = stone_2.speed.x * (stone_1.pos.y - stone_2.pos.y)
        - stone_2.speed.y * (stone_1.pos.x - stone_2.pos.x);

    let denominator = stone_2.speed.y * stone_1.speed.x - stone_2.speed.x * stone_1.speed.y;

    if denominator == 0.0 {
        return false; //lines paralel
    }

    let ua = numerator / denominator;

    let numerator = stone_1.speed.x * (stone_1.pos.y - stone_2.pos.y)
    - stone_1.speed.y * (stone_1.pos.x - stone_2.pos.x);

    // let denominator = stone_1.speed.y * stone_2.speed.x - stone_1.speed.x * stone_2.speed.y;

    let ub = numerator / denominator;

    if ua < -0.001 || ub < -0.001 {
        return false; //crossing in the past
    }

    let crossing_point = Coord {
        x: stone_1.pos.x + ua * stone_1.speed.x,
        y: stone_1.pos.y + ua * stone_1.speed.y,
        z: stone_1.pos.z + ua * stone_1.speed.z,
    };

    let inbounds = crossing_point.x >= bounds.0
        && crossing_point.x <= bounds.1
        && crossing_point.y >= bounds.0
        && crossing_point.y <= bounds.1;

    return inbounds;
}
pub fn run() {
    // let bounds = (7.0,27.0);
    let bounds = (200000000000000.0, 400000000000000.0);
    let hailstones: Vec<Hailstone> = include_str!("input")
        .lines()
        .map(|l| {
            l.split(" @ ")
                .map(|splt| {
                    splt.split(", ")
                        .map(|n| n.parse::<f64>().unwrap())
                        .collect::<Vec<f64>>()
                })
                .collect::<Vec<Vec<f64>>>()
        })
        .map(|hail_vec| Hailstone {
            pos: Coord {
                x: hail_vec[0][0],
                y: hail_vec[0][1],
                z: hail_vec[0][2],
            },
            speed: Coord {
                x: hail_vec[1][0],
                y: hail_vec[1][1],
                z: hail_vec[1][2],
            },
        }).collect();


    let mut colisions = 0;
    let mut total = 0;
    for i in 0..(hailstones.len() - 1){
        for j in (i + 1)..hailstones.len(){
            total += 1;
            if hail_collides(&hailstones[i], &hailstones[j], bounds){
                colisions += 1;
            }
        }
    }
    println!("{}", total);

    println!("{}", colisions);

    println!(
"(declare-const rpx Int)
(declare-const rpy Int)
(declare-const rpz Int)
(declare-const rsx Int)
(declare-const rsy Int)
(declare-const rsz Int)");


    for i in 0..6{
        println!("(declare-const t{i} Int)")
    }

    for i in 0..6{
        println!("(assert (= (+ rpx (* rsx t{i})) (+ {} (* {} t{i})) ))", hailstones[i].pos.x, hailstones[i].speed.x);
        println!("(assert (= (+ rpy (* rsy t{i})) (+ {} (* {} t{i})) ))", hailstones[i].pos.y, hailstones[i].speed.y);
        println!("(assert (= (+ rpz (* rsz t{i})) (+ {} (* {} t{i})) ))", hailstones[i].pos.z, hailstones[i].speed.z);

    }

    println!(
"(check-sat)
(get-model)");

println!();
println!("{}", 434470227085520 as usize+164429529509188+309721960025816)
}
