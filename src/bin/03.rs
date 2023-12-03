use std::collections::HashSet;

advent_of_code::solution!(3);

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn get_number(grid: &Vec<Vec<char>>, used: &mut HashSet<(usize, usize)>, i: usize, j: usize) -> u32 {
    let mut start: usize = j;
    let mut end: usize = j;
    while start > 0 && grid[i][start - 1].is_digit(10) { start -= 1; }
    while end < grid[i].len() && grid[i][end].is_digit(10) { end += 1; }

    if used.contains(&(i, start)) { return 0; }
    used.insert((i, start));

    grid[i].iter()
        .skip(start)
        .take(end - start)
        .fold(0, |acc, d| acc * 10 + d.to_digit(10).unwrap())
}

fn is_valid(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if !grid[i][j].is_digit(10) { return false; }

    if i > 0 && j > 0 { if !grid[i - 1][j - 1].is_digit(10) && grid[i - 1][j - 1] != '.' { return true; } }
    if i > 0 { if !grid[i - 1][j].is_digit(10) && grid[i - 1][j] != '.' { return true; } }
    if i > 0 && j < grid[i].len() - 1 { if !grid[i - 1][j + 1].is_digit(10) && grid[i - 1][j + 1] != '.' { return true; } }
    if j < grid[i].len() - 1 { if !grid[i][j + 1].is_digit(10) && grid[i][j + 1] != '.' { return true; } }
    if i < grid.len() - 1 && j < grid[i].len() - 1 { if !grid[i + 1][j + 1].is_digit(10) && grid[i + 1][j + 1] != '.' { return true; } }
    if i < grid.len() - 1 { if !grid[i + 1][j].is_digit(10) && grid[i + 1][j] != '.' { return true; } }
    if i < grid.len() - 1 && j > 0 { if !grid[i + 1][j - 1].is_digit(10) && grid[i + 1][j - 1] != '.' { return true; } }
    if j > 0 { if !grid[i][j - 1].is_digit(10) && grid[i][j - 1] != '.' { return true; } }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = parse(input);
    let mut used: HashSet<(usize, usize)> = HashSet::new();
    let mut ans: Option<u32> = None;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if is_valid(&grid, i, j) {
                ans = Some(ans.unwrap_or(0) + get_number(&grid, &mut used, i, j));
            }
        }
    }
    ans
}

fn get_gear_numbers(grid: &Vec<Vec<char>>, used: &mut HashSet<(usize, usize)>, i: usize, j: usize) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    if i > 0 && j > 0 { if grid[i - 1][j - 1].is_digit(10) { v.push(get_number(grid, used, i - 1, j - 1)); } }
    if i > 0 { if grid[i - 1][j].is_digit(10) { v.push(get_number(grid, used, i - 1, j)); } }
    if i > 0 && j < grid[i].len() - 1 { if grid[i - 1][j + 1].is_digit(10) { v.push(get_number(grid, used, i - 1, j + 1)); } }
    if j < grid[i].len() - 1 { if grid[i][j + 1].is_digit(10) { v.push(get_number(grid, used, i, j + 1)); } }
    if i < grid.len() - 1 && j < grid[i].len() - 1 { if grid[i + 1][j + 1].is_digit(10) { v.push(get_number(grid, used, i + 1, j + 1)); } }
    if i < grid.len() - 1 { if grid[i + 1][j].is_digit(10) { v.push(get_number(grid, used, i + 1, j)); } }
    if i < grid.len() - 1 && j > 0 { if grid[i + 1][j - 1].is_digit(10) { v.push(get_number(grid, used, i + 1, j - 1)); } }
    if j > 0 { if grid[i][j - 1].is_digit(10) { v.push(get_number(grid, used, i, j - 1)); } }
    v
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = parse(input);
    let mut used: HashSet<(usize, usize)> = HashSet::new();
    let mut ans: Option<u64> = None;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '*' {
                let gears: Vec<u32> = get_gear_numbers(&grid, &mut used, i, j).into_iter().filter(|n| *n > 0).collect();
                if gears.len() != 2 { continue; }
                ans = Some(ans.unwrap_or(0) + gears[0]  as u64 * gears[1] as u64);
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
