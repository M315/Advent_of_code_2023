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

fn parse(input: &str) -> (Option<(usize, usize)>, Vec<Vec<bool>>) {
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
    (start, grid)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (start, grid) = parse(input);
    let steps: usize = 64;

    assert!(start.is_some());

    Some(bfs(start.unwrap(), steps, &grid) as u32)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (start, grid) = parse(input);
    let steps: usize = 26_501_365;

    assert!(start.is_some());
    assert!(grid.len() == grid[0].len());
    assert!(start.unwrap().0 == start.unwrap().1 && start.unwrap().0 == grid.len() / 2);
    assert!(steps.rem_euclid(grid.len()) == grid.len() / 2);

    let size = grid.len();

    let grid_width = steps / size - 1;
    let odd = (grid_width / 2 * 2 + 1).pow(2);
    let even = ((grid_width + 1) / 2 * 2).pow(2);

    let odd_points = bfs(start.unwrap(), size * 2 + 1, &grid);
    let even_points = bfs(start.unwrap(), size * 2, &grid);

    // Corners
    let corner_up = bfs((0, start.unwrap().1), size - 1, &grid);
    let corner_down = bfs((size - 1, start.unwrap().1), size - 1, &grid);
    let corner_right = bfs((start.unwrap().0, 0), size - 1, &grid);
    let corner_left = bfs((start.unwrap().0, size - 1), size - 1, &grid);
    
    let small_corner_up_right = bfs((0, 0), size / 2 - 1, &grid);
    let small_corner_up_left = bfs((0, size - 1), size / 2 - 1, &grid);
    let small_corner_down_right = bfs((size - 1, 0), size / 2 - 1, &grid);
    let small_corner_down_left = bfs((size - 1, size - 1), size / 2 - 1, &grid);

    let large_corner_up_right = bfs((0, 0), size * 3 / 2 - 1, &grid);
    let large_corner_up_left = bfs((0, size - 1), size * 3 / 2 - 1, &grid);
    let large_corner_down_right = bfs((size - 1, 0), size * 3 / 2 - 1, &grid);
    let large_corner_down_left = bfs((size - 1, size - 1), size * 3 / 2 - 1, &grid);

    Some(
        odd * odd_points + even * even_points
        + corner_up + corner_down + corner_right + corner_left 
        + (grid_width + 1) * (small_corner_up_right + small_corner_up_left + small_corner_down_right + small_corner_down_left)
        + grid_width * (large_corner_up_right + large_corner_up_left  + large_corner_down_right + large_corner_down_left)
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }
}
