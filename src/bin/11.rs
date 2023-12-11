advent_of_code::solution!(11);

fn extend_space(map: Vec<Vec<char>>, extra_space: usize) -> Vec<Vec<char>> {
    let mut extended_map: Vec<Vec<char>> = Vec::new();
    // Extend rows
    for row in &map {
        extended_map.push(row.to_vec());
        if row.iter().filter(|&c| *c != '.').count() == 0 {
            extended_map.push(row.to_vec());
        }
    }
    // Extend columns
    let mut extra_columns: usize = 0;
    for j in 0..map[0].len() {
        let mut empty: bool = true;
        for i in 0..map.len() { empty &= map[i][j] == '.'; }
        if empty {
            for i in 0..extended_map.len() {
                extended_map[i].insert(j + extra_columns, '.');
            }
            extra_columns += 1;
        }
    }
    extended_map
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    map = extend_space(map, 2);
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }
    let mut dist: u32 = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            dist += (galaxies[i].0).abs_diff(galaxies[j].0) as u32 + (galaxies[i].1).abs_diff(galaxies[j].1) as u32;
        }
    }
    Some(dist)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
