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

fn energize(start: State, grid: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let mut energized: Vec<Vec<bool>> = vec![vec![false; grid.first().unwrap().len()]; grid.len()];
    let mut seen: HashSet<State> = HashSet::new();
    let mut q: Vec<State> = vec![start];

    while let Some(state) = q.pop() {
        energized[state.pos.0][state.pos.1] = true;
        match grid[state.pos.0][state.pos.1] {
            '.' => {
                if let Some(next_state) = state.next_state((grid.len(), grid[0].len())) {
                    if !seen.contains(&next_state) {
                        seen.insert(next_state);
                        q.push(next_state);
                    }
                }
            },
            '\\' => {
                if let Some(next_state) = state.reflect('\\', (grid.len(), grid[0].len())) {
                    if !seen.contains(&next_state) {
                        seen.insert(next_state);
                        q.push(next_state);
                    }
                }
            },
            '/' => {
                if let Some(next_state) = state.reflect('/', (grid.len(), grid[0].len())) {
                    if !seen.contains(&next_state) {
                        seen.insert(next_state);
                        q.push(next_state);
                    }
                }
            },
            '-' => {
                let next_steps: (Option<State>, Option<State>) = state.bifurcate('-', (grid.len(), grid[0].len()));
                if let Some(next_state) = next_steps.0 {
                    if !seen.contains(&next_state) {
                        seen.insert(next_state);
                        q.push(next_state);
                    }
                }
                if let Some(next_state) = next_steps.1 {
                    if !seen.contains(&next_state) {
                        seen.insert(next_state);
                        q.push(next_state);
                    }
                }
            },
            '|' => {
                let next_steps: (Option<State>, Option<State>) = state.bifurcate('|', (grid.len(), grid[0].len()));
                if let Some(next_state) = next_steps.0 {
                    if !seen.contains(&next_state) {
                        seen.insert(next_state);
                        q.push(next_state);
                    }
                }
                if let Some(next_state) = next_steps.1 {
                    if !seen.contains(&next_state) {
                        seen.insert(next_state);
                        q.push(next_state);
                    }
                }
            },
            _ => panic!("Invalid character"),
        }
    }
    energized
}

fn count_energy(energized: Vec<Vec<bool>>) -> u32 {
    energized.into_iter()
        .fold(0, |acc, row| acc + row.iter().filter(|&x| *x).count() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let energized: Vec<Vec<bool>> = energize( State{ pos: (0, 0), dir: Direction::Right }, &grid);
    Some(count_energy(energized))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut ans: u32 = 0;
    // Left & Right
    for i in 0..grid.len() {
        ans = ans.max(count_energy(energize( State{ pos: (i, 0), dir: Direction::Right }, &grid)));
        ans = ans.max(count_energy(energize( State{ pos: (i, grid[i].len() - 1), dir: Direction::Left }, &grid)));
    }
    // Up & Down
    for j in 0..grid[0].len() {
        ans = ans.max(count_energy(energize( State{ pos: (0, j), dir: Direction::Down }, &grid)));
        ans = ans.max(count_energy(energize( State{ pos: (grid.len() - 1, j), dir: Direction::Up }, &grid)));
    }
    Some(ans)
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
        assert_eq!(result, Some(51));
    }
}
