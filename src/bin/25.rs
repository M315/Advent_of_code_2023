use std::collections::{HashMap, HashSet};
use petgraph::graph::{NodeIndex, UnGraph, Graph};
use petgraph::data::FromElements;
use petgraph::dot::{Dot, Config};

advent_of_code::solution!(25);

#[derive(Debug)]
struct Node {
    n: usize,
    neighbours: Vec<usize>,
}

impl Node {
    fn new(input: &str, count: &mut usize, track: &mut HashMap<String, usize>) -> Self {
        let (name, neighbours) = input.split_once(": ").unwrap();
        if !track.contains_key(&name.to_string()) {
            track.insert(name.to_string(), *count);
            *count += 1;
        }
        
        let mut neigh: Vec<usize> = Vec::new();
        for neighbour in neighbours.split(" ") {
            if !track.contains_key(&neighbour.trim().to_string()) {
                track.insert(neighbour.trim().to_string(), *count);
                neigh.push(*count);
                *count += 1;
            } else {
                neigh.push(*track.get(&neighbour.trim().to_string()).unwrap());
            }
        }

        Self{ n: *track.get(&name.to_string()).unwrap(), neighbours: neigh }
    }
}

fn bfs(graph: &Vec<Vec<usize>>, forbiden: HashSet<(usize, usize)>) -> Option<u32> {
    let mut seen: Vec<bool> = vec![false; graph.len()];
    let mut q: Vec<usize> = Vec::new();

    seen[0] = true;
    q.push(0);

    while let Some(node) = q.pop() {
        for &neighbour in &graph[node] {
            if forbiden.contains(&(node, neighbour)) || forbiden.contains(&(neighbour, node)) {
                continue;
            }
            if !seen[neighbour] {
                seen[neighbour] = true;
                q.push(neighbour);
            }
        }
    }
    
    let partition: usize = seen.iter().filter(|&b| *b).count();
    if partition == graph.len() { return None; }
    Some((partition * (graph.len() - partition)) as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut track: HashMap<String, usize> = HashMap::new();
    let mut count: usize = 0;
    let nodes: Vec<Node> = input.lines()
        .map(|line| Node::new(line, &mut count, &mut track))
        .collect();
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); count];
    let mut edges: Vec<(usize, usize)> = Vec::new();
    for node in &nodes {
        for &neighbour in &node.neighbours {
            edges.push((node.n, neighbour));
            graph[node.n].push(neighbour);
            graph[neighbour].push(node.n);
        }
    }
    let edges: Vec<_> = edges.into_iter()
        .map(|(a, b)| (NodeIndex::new(a), NodeIndex::new(b)))
        .collect::<Vec<_>>();
    let mut g: Graph<_, ()> = Graph::<_, ()>::new();
    for i in 0..nodes.len() {
        g.add_node(i);
    }
    g.extend_with_edges(&edges);
    println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
    bfs(&graph, HashSet::from([(1455, 1030), (1071, 15), (647, 24)]))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}
