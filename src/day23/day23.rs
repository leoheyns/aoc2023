use std::{collections::{HashMap, HashSet}, vec};

fn findlongest(node: (usize, usize), end:(usize, usize), edges: &HashMap<(usize, usize), Vec<((usize, usize), usize)>>) -> usize{
    if node == end{
        return 0;
    }
    return edges.get(&node).unwrap().iter().map(|(next_node, cost)| cost + findlongest(*next_node, end, edges)).max().unwrap();
}

fn dfs_longest(node: (usize, usize), end:(usize, usize), edges: &HashMap<(usize, usize), Vec<((usize, usize), usize)>>, visited: HashSet<(usize, usize)>) -> usize{
    if node == end{
        return 0;
    }
    let mut current_max = 0;
    for (to, cost) in edges.get(&node).unwrap(){
        if !visited.contains(to){
            let mut next_visited = visited.clone();
            next_visited.insert(*to);
            let to_cost = dfs_longest(*to, end, edges, next_visited) + cost;
            current_max = std::cmp::max(to_cost, current_max);
        }
    }
    return current_max;
}
pub fn run() {
    let grid = include_str!("input")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut nodes: HashSet<(usize, usize)> = HashSet::new();
    let mut edges: HashMap<(usize, usize), Vec<((usize, usize), usize)>> = HashMap::new(); //from, (to, cost)
    for i in 1..(grid.len() - 1) {
        for j in 1..(grid[0].len()) {
            if grid[i][j] == '.' {
                let mut path_options = 0;
                for (ioffset, joffset) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    if grid[(i as i32 + ioffset) as usize][(j as i32 + joffset) as usize] != '#' {
                        path_options += 1;
                    }
                }
                if path_options > 2 {
                    nodes.insert((i, j));
                    // for k in 0..3{
                    //     for l in 0..3{
                    //         print!("{}", grid[i+k-1][j+l-1])
                    //     }
                    //     println!()
                    // }
                    // println!()
                }
            }
        }
    }
    let start = (0, 1);
    let end = (grid.len()-1, grid[0].len() - 2);
    nodes.insert(start);
    nodes.insert(end);

    for node in &nodes {
        if *node == end {
            continue;
        }
        for offsets in [(1, 0), (0, 1)] {
            let mut current_coord = (node.0 + offsets.0, node.1 + offsets.1);

            if grid[current_coord.0][current_coord.1] == '#' {
                continue;
            }
            let mut camefrom = node.clone();
            let mut edge_length = 0;

            while !nodes.contains(&current_coord) {
                for coord_offset in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let candidatecoord = (
                        (current_coord.0 as i32 + coord_offset.0) as usize,
                        (current_coord.1 as i32 + coord_offset.1) as usize,
                    );
                    if camefrom != candidatecoord && grid[candidatecoord.0][candidatecoord.1] != '#'
                    {
                        edge_length += 1;
                        camefrom = current_coord;
                        current_coord = candidatecoord;
                        break;
                    }
                }
            }
            if !edges.contains_key(node){
                edges.insert(*node, vec![]);
            }
            // edges.insert(*node, (current_coord, edge_length));
            edges.get_mut(node).unwrap().push((current_coord, edge_length + 1))
        }
    }

    let part1 = findlongest(start, end, &edges);
    println!("{}", part1);

    let mut new_edges: Vec<((usize, usize), (usize, usize), usize)> = vec![];

    for (from, tos) in &edges{
        for (to, cost) in tos{
            new_edges.push((*to, *from, *cost));
        }
    }

    let mut symedges = edges.clone();

    for (from, to, cost) in new_edges{
        if from != end{
            symedges.get_mut(&from).unwrap().push((to, cost))
        }
    }

    let part2 = dfs_longest(start, end, &symedges, HashSet::from([start]));

    println!("part 2 {}", part2);


}
