use std::collections::HashMap;
advent_of_code::solution!(14);

fn tilt_north(grid: &mut Vec<Vec<char>>) -> Option<u32> {
    let mut changes: Option<u32> = None;
    for i in 0..grid.len() - 1 {
        for j in 0..grid[0].len() {
            if grid[i][j] == '.' && grid[i + 1][j] == 'O' {
                grid[i][j] = 'O';
                grid[i + 1][j] = '.';
                changes = Some(changes.unwrap_or(0) + 1);
            }
        }
    }
    changes
}

fn tilt_south(grid: &mut Vec<Vec<char>>) -> Option<u32> {
    let mut changes: Option<u32> = None;
    for i in (1..grid.len()).rev() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '.' && grid[i - 1][j] == 'O' {
                grid[i][j] = 'O';
                grid[i - 1][j] = '.';
                changes = Some(changes.unwrap_or(0) + 1);
            }
        }
    }
    changes
}

fn tilt_west(grid: &mut Vec<Vec<char>>) -> Option<u32> {
    let mut changes: Option<u32> = None;
    for j in 0..grid[0].len() - 1  {
        for i in 0..grid.len() {
            if grid[i][j] == '.' && grid[i][j + 1] == 'O' {
                grid[i][j] = 'O';
                grid[i][j + 1] = '.';
                changes = Some(changes.unwrap_or(0) + 1);
            }
        }
    }
    changes
}

fn tilt_east(grid: &mut Vec<Vec<char>>) -> Option<u32> {
    let mut changes: Option<u32> = None;
    for j in (1..grid[0].len()).rev()  {
        for i in 0..grid.len() {
            if grid[i][j] == '.' && grid[i][j - 1] == 'O' {
                grid[i][j] = 'O';
                grid[i][j - 1] = '.';
                changes = Some(changes.unwrap_or(0) + 1);
            }
        }
    }
    changes
}

fn spin_cycle(grid: &mut Vec<Vec<char>>) {
    while tilt_north(grid).is_some() { }
    while tilt_west(grid).is_some() { }
    while tilt_south(grid).is_some() { }
    while tilt_east(grid).is_some() { }
}

fn total_load(grid: &Vec<Vec<char>>) -> Option<u32> {
    let len: u32 = grid.len() as u32;
    grid.into_iter()
        .enumerate()
        .fold(None, |acc, (i, row)| Some(acc.unwrap_or(0) + (len - i as u32) * row.iter().filter(|&c| *c == 'O').count() as u32))

}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    while tilt_north(&mut grid).is_some() { }
    total_load(&mut grid)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut seen: HashMap<Vec<Vec<char>>, u32> = HashMap::new();
    let mut count: u32 = 0;
    while !seen.contains_key(&grid) {
        seen.insert(grid.to_vec(), count);
        spin_cycle(&mut grid);
        count += 1;
    }
    let cycle: (u32, u32) = (*seen.get(&grid).unwrap(), count - *seen.get(&grid).unwrap());

    let needed_cycles: u32 = (1_000_000_000 - cycle.0).rem_euclid(cycle.1);
    for _ in 0..needed_cycles {
        spin_cycle(&mut grid);
    }
    total_load(&mut grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
