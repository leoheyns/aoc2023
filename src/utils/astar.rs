use std::{
    collections::{HashMap, BinaryHeap, HashSet},
    hash::Hash, vec,
};

pub trait Astarcontext<State>
where
    State: Hash + Eq,
{
    fn start(&self) -> State;

    fn neighbours(&self, state: &State) -> Vec<(State, usize)>;

    fn heuristic(&self, state: &State) -> usize;

    fn goal(&self, state: &State) -> bool;
}

fn reconstruct_path<State>(camefrom: HashMap<State, State>, dest: State) -> Vec<State>
where
    State: Hash + Eq + Copy,
{
    let mut result: Vec<State> = vec![];
    let mut current = dest;
    while camefrom.contains_key(&current){
        result.push(current);
        current = *camefrom.get(&current).unwrap();
    }
    return result;
}

pub fn a_star<State>(context: &impl Astarcontext<State>) -> (usize, Vec<State>)
where
    State: Hash + Eq + Copy,
{

    #[derive(PartialEq, Eq)]
    struct QueueState<T>
    {
        prio: usize,
        state: T,
    }
    
    impl<T> Ord for QueueState<T> 
    where T: Hash + Eq + Copy
    {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.prio.cmp(&other.prio).reverse()
        }
    }

    impl<T> PartialOrd for QueueState<T> 
    where T: Hash + Eq + Copy
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut visited: HashSet<State> = HashSet::new();
    
    let start = context.start();
    let mut queue: BinaryHeap<QueueState<State>> = BinaryHeap::new();

    let mut gscore: HashMap<State, usize> = HashMap::new();


    gscore.insert(start, 0);

    let mut fscore: HashMap<State, usize> = HashMap::new();
    fscore.insert(start, context.heuristic(&start));

    queue.push(QueueState { prio: *fscore.get(&start).unwrap(), state: start });

    let mut camefrom: HashMap<State, State> = HashMap::new();


    while queue.len() > 0 {

        let current = queue.pop().unwrap().state;

        if context.goal(&current) {
            return (*gscore.get(&current).unwrap(), reconstruct_path(camefrom, current));
        }

        for (neighbour, cost) in context.neighbours(&current) {
            let tentative_gscore: usize = gscore.get(&current).unwrap() + cost;

            if tentative_gscore < *gscore.get(&neighbour).unwrap_or(&usize::MAX) {
                camefrom.insert(neighbour, current);
                gscore.insert(neighbour, tentative_gscore);
                let new_fscore = tentative_gscore + context.heuristic(&neighbour);
                fscore.insert(neighbour, new_fscore);
                // queue.retain(|s| s.state != neighbour);
                if !visited.contains(&neighbour){
                    visited.insert(neighbour);
                    queue.push(QueueState { prio: new_fscore, state: neighbour });
                }
            }
        }
    }
    return (0, vec![]);
}