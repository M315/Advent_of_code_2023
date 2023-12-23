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

    fn length(&self) -> usize {
        self.steps.iter()
            .fold(0, |count, row| count + row.iter().filter(|&b| *b).count())
    }
}

fn longest_bfs(start: (usize, usize), grid: &Vec<Vec<char>>) -> Option<Path> {
    let mut q: Vec<Path> = vec![Path::new(start, grid)];
    let mut longest_path: Option<Path> = None;
    let mut longest: Vec<Vec<usize>> = vec![vec![0; grid[0].len()]; grid.len()];

    while let Some(path) = q.pop() {
        if path.pos == (grid.len() - 1, grid[0].len() - 2) {
            match longest_path {
                None => longest_path = Some(path.clone()),
                Some(ref long) => { if long.length < path.length { longest_path = Some(path); } }
            }
            continue;
        }
        for next_pos in path.next_positions(grid) {
            if longest[next_pos.pos.0][next_pos.pos.1] < next_pos.length {
                longest[next_pos.pos.0][next_pos.pos.1] = next_pos.length;
                q.push(next_pos);
            }
        }
    }

    longest_path
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    Some(longest_bfs((0, 1), &grid).unwrap().length() as u32 - 1)
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
    Some(longest_bfs((0, 1), &grid).unwrap().length() as u32 - 1)
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
