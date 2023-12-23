advent_of_code::solution!(23);

#[derive(Debug, Clone)]
struct Path {
    pos: (usize, usize),
    steps: Vec<Vec<bool>>,
    length: usize,
}

impl Path {
    fn new(pos: (usize, usize), grid: &Vec<Vec<char>>) -> Self {
        let mut steps: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
        steps[pos.0][pos.1] = true;
        Self { pos, steps, length: 0 }
    }

    fn next_positions(&self, grid: &Vec<Vec<char>>) -> Vec<Self> {
        let mut valid_path: Vec<Path> = Vec::new();

        if self.pos.0 > 0 && !self.steps[self.pos.0 - 1][self.pos.1] {
            let mut path = self.clone();
            path.pos.0 = self.pos.0 - 1;
            path.steps[path.pos.0][path.pos.1] = true;
            path.length += 1;
            match grid[path.pos.0][path.pos.1] {
                '.' | '^' => valid_path.push(path),
                _ => {},
            }
        }
        if self.pos.0 < grid.len() - 1 && !self.steps[self.pos.0 + 1][self.pos.1] {
            let mut path = self.clone();
            path.pos.0 = self.pos.0 + 1;
            path.steps[path.pos.0][path.pos.1] = true;
            path.length += 1;
            match grid[path.pos.0][path.pos.1] {
                '.' | 'v' => valid_path.push(path),
                _ => {},
            }
        }
        if self.pos.1 > 0 && !self.steps[self.pos.0][self.pos.1 - 1] {
            let mut path = self.clone();
            path.pos.1 = self.pos.1 - 1;
            path.steps[path.pos.0][path.pos.1] = true;
            path.length += 1;
            match grid[path.pos.0][path.pos.1] {
                '.' | '<' => valid_path.push(path),
                _ => {},
            }
        }
        if self.pos.1 < grid[0].len() - 1 && !self.steps[self.pos.0][self.pos.1 + 1] {
            let mut path = self.clone();
            path.pos.1 = self.pos.1 + 1;
            path.steps[path.pos.0][path.pos.1] = true;
            path.length += 1;
            match grid[path.pos.0][path.pos.1] {
                '.' | '>' => valid_path.push(path),
                _ => {},
            }
        }

        valid_path
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Edge {
    node: usize,
    length: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    pos: (usize, usize),
    neighbours: Vec<Edge>,
}

impl Node {
    fn new(pos: (usize, usize)) -> Self {
        Self { pos, neighbours: Vec::new() }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct PathGraph {
    node: usize,
    length: usize,
    visited: Vec<bool>,
}

fn find_neighbours(node: &mut Node, nodes: &Vec<Node>, grid: &Vec<Vec<char>>) {
    let mut q: Vec<Path> = vec![Path::new(node.pos, grid)];

    while let Some(path) = q.pop() {
        for next_pos in path.next_positions(grid) {
            let mut found: bool = false;
            for (i, neighbour) in nodes.iter().enumerate() {
                if next_pos.pos == neighbour.pos && next_pos.pos != node.pos {
                    node.neighbours.push(Edge { node: i, length: next_pos.length });
                    found = true;
                }
            }
            if !found { q.push(next_pos); }
        }
    }
}

fn fold(grid: &Vec<Vec<char>>) -> Vec<Node> {
    // Find intersection Points
    let mut nodes: Vec<Node> = vec![Node::new((0, 1))];
    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            if grid[i][j] != '#' && vec![grid[i - 1][j], grid[i + 1][j], grid[i][j - 1], grid[i][j + 1]].iter().filter(|&c| *c != '#').count() > 2 {
                nodes.push(Node::new((i, j)));
            }
        }
    }
    nodes.push(Node::new((grid.len() - 1, grid[0].len() - 2)));

    // Fill neighbours
    let nodes_copy = nodes.clone();
    for node in nodes.iter_mut() {
        find_neighbours(node, &nodes_copy, &grid);
    }

    nodes
}

fn longest_path(nodes: &Vec<Node>) -> Option<u32> {
    let mut longest: Option<u32> = None;
    let mut q: Vec<PathGraph> = vec![PathGraph { node: 0, length: 0, visited: vec![false; nodes.len()] }];
    q[0].visited[0] = true;

    while let Some(path) = q.pop() {
        if path.node == nodes.len() - 1 {
            longest = match longest {
                None => Some(path.length as u32),
                Some(len) => Some(len.max(path.length as u32)),
            };
        }
        for &edge in nodes[path.node].neighbours.iter() {
            if path.visited[edge.node] { continue; }
            let mut next_step =  PathGraph {
                length: path.length + edge.length,
                node: edge.node,
                visited: path.visited.clone()
            };
            next_step.visited[edge.node] = true;
            q.push(next_step);
        }
    }

    longest
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    let nodes = fold(&grid);
    Some(longest_path(&nodes).unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars()
            .map(|c| match c {
                '.' | '#' => c,
                _ => '.',
            })
            .collect())
        .collect();
    let nodes = fold(&grid);
    Some(longest_path(&nodes).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
