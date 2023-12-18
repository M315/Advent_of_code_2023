use std::cmp::Ordering;
use std::collections::BinaryHeap;

advent_of_code::solution!(17);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn as_usize(&self) -> usize {
        match self {
            Direction::Down => 1,
            Direction::Up => 2,
            Direction::Left => 3,
            Direction::Right => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    cost: u32,
    pos: (usize, usize),
    dir: Option<Direction>,
    step_strike: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn next_states(&self, grid: &Vec<Vec<u32>>) -> Vec<Self> {
        let mut new_states: Vec<Self> = Vec::new();
        match self.dir {
            None => {
                self.add_state(Direction::Down, grid, &mut new_states);
                self.add_state(Direction::Up, grid, &mut new_states);
                self.add_state(Direction::Left, grid, &mut new_states);
                self.add_state(Direction::Right, grid, &mut new_states);
            },
            Some(Direction::Left) => {
                if self.step_strike >= 4 {
                    self.add_state(Direction::Down, grid, &mut new_states);
                    self.add_state(Direction::Up, grid, &mut new_states);
                }
                if self.step_strike < 10 { self.add_state(Direction::Left, grid, &mut new_states); }
            },
            Some(Direction::Right) => {
                if self.step_strike >= 4 {
                    self.add_state(Direction::Down, grid, &mut new_states);
                    self.add_state(Direction::Up, grid, &mut new_states);
                }
                if self.step_strike < 10 { self.add_state(Direction::Right, grid, &mut new_states); }
            },
            Some(Direction::Up) => {
                if self.step_strike >= 4 {
                    self.add_state(Direction::Right, grid, &mut new_states);
                    self.add_state(Direction::Left, grid, &mut new_states);
                }
                if self.step_strike < 10 { self.add_state(Direction::Up, grid, &mut new_states); }
            },
            Some(Direction::Down) => {
                if self.step_strike >= 4 {
                    self.add_state(Direction::Right, grid, &mut new_states);
                    self.add_state(Direction::Left, grid, &mut new_states);
                }
                if self.step_strike < 10 { self.add_state(Direction::Down, grid, &mut new_states); }
            },
        }
        new_states
    }

    fn add_state(&self, direction: Direction, grid: &Vec<Vec<u32>>, new_states: &mut Vec<Self>) {
        let strike: u32 = match self.dir == Some(direction) {
            false => 1,
            true => self.step_strike + 1,
        };
        let mut cost: u32 = self.cost;
        let mut pos: (usize, usize) = self.pos;
        match direction {
            Direction::Down => { 
                if self.pos.0 == grid.len() - 1 { return; }
                cost += grid[self.pos.0 + 1][self.pos.1];
                pos.0 += 1;
            },
            Direction::Up => { 
                if self.pos.0 == 0 { return; }
                cost += grid[self.pos.0 - 1][self.pos.1];
                pos.0 -= 1;
            },
            Direction::Right => { 
                if self.pos.1 == grid.first().unwrap().len() - 1 { return; }
                cost += grid[self.pos.0][self.pos.1 + 1];
                pos.1 += 1;
            },
            Direction::Left => { 
                if self.pos.1 == 0 { return; }
                cost += grid[self.pos.0][self.pos.1 - 1];
                pos.1 -= 1;
            },
        }
        new_states.push(Self {
            cost: cost,
            pos: pos,
            dir: Some(direction),
            step_strike: strike,
        })
    }

    fn update_dist(&self, dist: &mut Vec<Vec<Vec<Vec<u32>>>>) -> bool {
        let k: usize = match self.dir {
            None => 0,
            Some(dir) => dir.as_usize(),
        };
        if dist[self.pos.0][self.pos.1][k][self.step_strike as usize] <= self.cost { return false; }
        dist[self.pos.0][self.pos.1][k][self.step_strike as usize] = dist[self.pos.0][self.pos.1][k][self.step_strike as usize].min(self.cost);
        true
    }

    fn valid(&self, dist: &Vec<Vec<Vec<Vec<u32>>>>) -> bool {
        let k: usize = match self.dir {
            None => 0,
            Some(dir) => dir.as_usize(),
        };
        dist[self.pos.0][self.pos.1][k][self.step_strike as usize] >= self.cost
    }
}


fn dijkstra(start: State, goal: (usize, usize), grid: &Vec<Vec<u32>>) -> Option<u32> {
    let mut dist: Vec<Vec<Vec<Vec<u32>>>> = vec![vec![vec![vec![u32::MAX; 11]; 5]; grid[0].len()]; grid.len()];
    let mut q: BinaryHeap<State> = BinaryHeap::new();
    
    start.update_dist(&mut dist);
    q.push(start);

    while let Some(state) = q.pop() {
        if state.pos == goal {
            return Some(state.cost);
        }
        if !state.valid(&dist) { continue; }
        for next_state in state.next_states(&grid) {
            if next_state.update_dist(&mut dist) {
                q.push(next_state);
            }
        }
    }

    None
}


pub fn part_one(_input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    dijkstra(State{ cost: 0, pos: (0, 0), dir: None, step_strike: 0}, (grid.first().unwrap().len() - 1, grid.len() - 1), &grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
