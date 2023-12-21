use std::collections::HashSet;

advent_of_code::solution!(21);

fn bfs(start: (usize, usize), steps: usize, grid: &Vec<Vec<bool>>) -> usize {
    let mut current: Vec<(usize, usize)> = vec![start];

    for _ in 0..steps {
        let mut next_steps: HashSet<(usize, usize)> = HashSet::new();
        for (i, j) in current {
            if i > 0 && grid[i - 1][j] { next_steps.insert((i - 1, j)); }
            if i < grid.len() - 1 && grid[i + 1][j] { next_steps.insert((i + 1, j)); }
            if j > 0 && grid[i][j - 1] { next_steps.insert((i, j - 1)); }
            if j < grid[0].len() - 1 && grid[i][j + 1] { next_steps.insert((i, j + 1)); }
        }
        current = next_steps.into_iter().collect();
    }

    current.len()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut start: Option<(usize, usize)> = None;
    let grid: Vec<Vec<bool>> = input.lines().enumerate()
        .map(|(i, line)| line.chars()
            .enumerate()
            .map(|(j, c)| {
                match c {
                    '.' => true,
                    'S' => { start = Some((i, j)); true },
                    '#' => false,
                    _ => panic!("Invalid character"),
                }
            })
            .collect()
        )
        .collect();
    assert!(start.is_some());
    Some(bfs(start.unwrap(), 64, &grid) as u32)
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
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
