use std::{collections::{HashMap, HashSet}, vec};


fn mincutphase(graph: &Vec<Vec<u32>>, a: usize) -> ((usize,usize), u32){
    let mut g = graph.clone();
    let mut last = 0;
    let mut second_to_last = 0;
    let mut cutlenght = 0;
    while (0..g.len()).filter(|i| g[*i][g[0].len() - 1] != 0).count() > 1{
        second_to_last = last;
        last = (0..g.len()).max_by_key(|i| g[*i][a]).unwrap(); //index of line to join
        cutlenght = g[a][last];


        for i in 0..g.len(){
            g[i][a] = g[i][last] + g[i][a];
            g[i][last] = 0;
        }
        for j in 0..g[0].len(){
            g[a][j] =  g[a][j] + g[last][j];
            g[last][j] = 0;
        }
        g[a][a] = 0;

    }
    return ((second_to_last, last), cutlenght)
}

fn mincut(graph: Vec<Vec<u32>>){
    let mut g = graph.clone();
    while (0..g.len()).filter(|i| g[*i][g[0].len() - 1] != 0).count() > 1{
        let a = 0;
        let ((s,t), cutlenght) = mincutphase(&g, a);
        // println!("{}", cutlenght);
        if cutlenght == 3{
            let cutgroup =  g[t][g[t].len() - 1];
            println!("{}", cutgroup * (g.len() as u32 - cutgroup));
            return;
        }

        for i in 0..g.len(){
            g[i][s] = g[i][t] + g[i][s];
            g[i][t] = 0;
        }
    
        for j in 0..g[0].len(){
            g[s][j] =  g[t][j] + g[s][j];
            g[t][j] = 0;
        }
        g[t][s] = 0;
    }
}

pub fn run() {
    let edges:Vec<(&str, &str)> = include_str!("input")
        .lines()
        .map(|l| l.split(": "))
        .map(|mut splt| {
            (
                splt.next().unwrap(),
                splt.next().unwrap().split(" "),
            )
        }).flat_map(|(from, tos)| tos.map(move |t| (from, t))).collect::<Vec<(&str, &str)>>();

    let mut nodes = HashSet::new();
    for (from, to) in &edges{
        nodes.insert(*from);
        nodes.insert(*to);
    }

    let mut nodemap: HashMap<&str, usize> = HashMap::new();

    for (index, name) in (&nodes).iter().enumerate(){
        nodemap.insert(name, index);
    }

    let mut graphmatrix = vec![vec![0; nodes.len() + 1];nodes.len()];

    for i in 0..graphmatrix.len(){
        graphmatrix[i][nodes.len()] = 1 //aggregated node count, starts at 1 as no nodes are aggregated
    }

    for (from, to) in edges{
        let i = nodemap.get(from).unwrap();
        let j = nodemap.get(to).unwrap();
        graphmatrix[*i][*j] = 1;
        graphmatrix[*j][*i] = 1;
    }

    // graphmatrix = vec![vec![1,0,0,1];3];

    mincut(graphmatrix);

}