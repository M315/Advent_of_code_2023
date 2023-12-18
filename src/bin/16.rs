use std::collections::HashSet;

advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    pos: (usize, usize),
    dir: Direction,
}

impl State {
    fn next_state(&self, limits: (usize, usize)) -> Option<Self> {
        let mut pos: (usize, usize) = self.pos;
        match self.dir {
            Direction::Down => {
                if self.pos.0 >= limits.0 - 1 { return None; }
                pos.0 += 1;
            }
            Direction::Up => {
                if self.pos.0 == 0 { return None; }
                pos.0 -= 1;
            }
            Direction::Right => {
                if self.pos.1 >= limits.1 - 1 { return None; }
                pos.1 += 1;
            }
            Direction::Left => {
                if self.pos.1 == 0 { return None; }
                pos.1 -= 1;
            }
        }
        Some(Self { pos: pos, dir: self.dir })
    }

    fn reflect(&self, reflection: char, limits: (usize, usize)) -> Option<Self> {
        match reflection {
            '/' => {
                match self.dir {
                    Direction::Down => State{ pos: self.pos, dir: Direction::Left }.next_state(limits),
                    Direction::Up => State{ pos: self.pos, dir: Direction::Right }.next_state(limits),
                    Direction::Right => State{ pos: self.pos, dir: Direction::Up }.next_state(limits),
                    Direction::Left => State{ pos: self.pos, dir: Direction::Down }.next_state(limits),
                }
            },
            '\\' => {
                match self.dir {
                    Direction::Down => State{ pos: self.pos, dir: Direction::Right }.next_state(limits),
                    Direction::Up => State{ pos: self.pos, dir: Direction::Left }.next_state(limits),
                    Direction::Right => State{ pos: self.pos, dir: Direction::Down }.next_state(limits),
                    Direction::Left => State{ pos: self.pos, dir: Direction::Up }.next_state(limits),
                }
            }
            _ => panic!("Invalid reflection"),
        }
    }

    fn bifurcate(&self, bifurcation: char, limits: (usize, usize)) -> (Option<Self>, Option<Self>) {
        match bifurcation {
            '|' => {
                match self.dir {
                    Direction::Down | Direction::Up => (State{ pos: self.pos, dir: self.dir }.next_state(limits), None),
                    Direction::Right | Direction::Left => (State{ pos: self.pos, dir: Direction::Up }.next_state(limits), State{ pos: self.pos, dir: Direction::Down }.next_state(limits)),
                }
            },
            '-' => {
                match self.dir {
                    Direction::Down | Direction::Up => (State{ pos: self.pos, dir: Direction::Right }.next_state(limits), State{ pos: self.pos, dir: Direction::Left }.next_state(limits)),
                    Direction::Right | Direction::Left => (State{ pos: self.pos, dir: self.dir }.next_state(limits), None),
                }
            }
            _ => panic!("Invalid reflection"),
        }
    }
}

fn energize(state: State, grid: &Vec<Vec<char>>, energized: &mut Vec<Vec<bool>>, seen: &mut HashSet<State>) {
    //println!("{:?}, {:?}", state, seen.len());
    if seen.contains(&state) { return; }
    seen.insert(state.clone());
    energized[state.pos.0][state.pos.1] = true;

    match grid[state.pos.0][state.pos.1] {
        '.' => {
            if let Some(next_state) = state.next_state((grid.len(), grid[0].len())) {
                energize(next_state, grid, energized, seen);
            }
        },
        '\\' => {
            if let Some(next_state) = state.reflect('\\', (grid.len(), grid[0].len())) {
                energize(next_state, grid, energized, seen);
            }
        },
        '/' => {
            if let Some(next_state) = state.reflect('/', (grid.len(), grid[0].len())) {
                energize(next_state, grid, energized, seen);
            }
        },
        '-' => {
            let next_steps: (Option<State>, Option<State>) = state.bifurcate('-', (grid.len(), grid[0].len()));
            if let Some(next_state) = next_steps.0 { energize(next_state, grid, energized, seen); }
            if let Some(next_state) = next_steps.1 { energize(next_state, grid, energized, seen); }
        },
        '|' => {
            let next_steps: (Option<State>, Option<State>) = state.bifurcate('|', (grid.len(), grid[0].len()));
            if let Some(next_state) = next_steps.0 { energize(next_state, grid, energized, seen); }
            if let Some(next_state) = next_steps.1 { energize(next_state, grid, energized, seen); }
        },
        _ => panic!("Invalid character"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut energized: Vec<Vec<bool>> = vec![vec![false; grid.first().unwrap().len()]; grid.len()];
    energize(State{ pos: (0, 0), dir: Direction::Right }, &grid, &mut energized, &mut HashSet::new());
    energized.into_iter()
        .fold(None, |acc, row| Some(acc.unwrap_or(0) + row.iter().filter(|&x| *x).count() as u32))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
