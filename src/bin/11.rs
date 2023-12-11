advent_of_code::solution!(11);

fn extend_space(galaxies: &mut Vec<(usize, usize)>, map: Vec<Vec<char>>, extra_space: usize) {
    // Extend rows
    let mut extra_rows: usize = 0;
    for (i, row) in map.iter().enumerate() {
        if row.iter().filter(|&c| *c != '.').count() == 0 {
            for galaxy in galaxies.iter_mut() {
                if galaxy.0 > i + extra_rows { galaxy.0 += extra_space - 1; }
            }
            extra_rows += extra_space - 1;
        }
    }
    // Extend columns
    let mut extra_columns: usize = 0;
    for j in 0..map[0].len() {
        let mut empty: bool = true;
        for i in 0..map.len() { empty &= map[i][j] == '.'; }
        if empty {
            for galaxy in galaxies.iter_mut() {
                if galaxy.1 > j + extra_columns { galaxy.1 += extra_space - 1; }
            }
            extra_columns += extra_space - 1;
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }
    extend_space(&mut galaxies, map, 2);
    let mut dist: u64 = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            dist += (galaxies[i].0).abs_diff(galaxies[j].0) as u64 + (galaxies[i].1).abs_diff(galaxies[j].1) as u64;
        }
    }
    Some(dist)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }
    extend_space(&mut galaxies, map, 1000000);
    let mut dist: u64 = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            dist += (galaxies[i].0).abs_diff(galaxies[j].0) as u64 + (galaxies[i].1).abs_diff(galaxies[j].1) as u64;
        }
    }
    Some(dist)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}

