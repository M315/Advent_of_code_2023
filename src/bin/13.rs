advent_of_code::solution!(13);

struct Pattern {
    grid: Vec<Vec<char>>,
}

impl Pattern {
    fn new(input: &str) -> Self {
        Self {
            grid: input.lines()
                    .map(|line| line.chars().collect())
                    .collect()
        }
    }

    fn transpose(&self) -> Self {
        let len = self.grid.first().unwrap().len();
        let transposed_pattern: Vec<Vec<char>> = (0..len).into_iter()
            .map(|i| self.grid.iter().map(|row| row[i]).collect())
            .collect();
        Self {
            grid: transposed_pattern,
        }
    }

    fn clean_smudge(&mut self, i: usize, j: usize) {
        match self.grid[i][j] {
            '.' => self.grid[i][j] = '#',
            '#' => self.grid[i][j] = '.',
            _ => panic!("Invalid character"),
        }
    }

    fn mirror_position(&self, ignore: Option<usize>) -> Option<usize> {
        position_horizontal_mirror(self.grid.iter().map(|row| row.iter().collect::<String>()).collect(), ignore)
    }
}

fn position_horizontal_mirror<T: Eq>(rows: Vec<T>, ignore: Option<usize>) -> Option<usize> {
    for i in 0..rows.len() - 1 {
        if ignore.is_some() && Some(i + 1) == ignore { continue; }
        if rows[i] == rows[i + 1] {
            let mut mirror: bool = true;
            for j in 1..=i {
                if i + 1 + j >= rows.len() { break; }
                mirror &= rows[i - j] == rows[i + 1 + j];
            }
            if mirror { return Some(i + 1); }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let patterns: Vec<Pattern> = input.split("\n\n").map(|pattern| Pattern::new(pattern)).collect();
    patterns.into_iter()
        .fold(Some(0), |acc, pattern| Some(acc.unwrap() + 100 * pattern.mirror_position(None).unwrap_or(0) + pattern.transpose().mirror_position(None).unwrap_or(0)))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut patterns: Vec<Pattern> = input.split("\n\n").map(|pattern| Pattern::new(pattern)).collect();
    let mut ans: usize = 0;
    for (k, pattern) in patterns.iter_mut().enumerate() {
        let original_mirror: (Option<usize>, Option<usize>) = (pattern.mirror_position(None), pattern.transpose().mirror_position(None));
        let mut found: bool = false;
        for i in 0..pattern.grid.len() {
            if found { break; }
            for j in 0..pattern.grid[0].len() {
                pattern.clean_smudge(i, j);

                let new_mirror: (Option<usize>, Option<usize>) = (pattern.mirror_position(original_mirror.0), pattern.transpose().mirror_position(original_mirror.1));

                if new_mirror == (None, None) {
                    pattern.clean_smudge(i, j);
                    continue;
                }
                match (original_mirror.0 == new_mirror.0, original_mirror.1 == new_mirror.1) {
                    (false, false) => {
                            ans += 100 * new_mirror.0.unwrap_or(0) + new_mirror.1.unwrap_or(0);
                            found = true; 
                            break;
                        }
                    (true, false) => {
                            ans += new_mirror.1.unwrap(); 
                            found = true; 
                            break;
                        }
                    (false, true) => {
                            ans += 100 * new_mirror.0.unwrap(); 
                            found = true; 
                            break;
                        }
                    (true, true) => { pattern.clean_smudge(i, j); }
                }
            }
        }
        if !found { panic!("{} not found!", k); }
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY).replace("\r", ""));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY).replace("\r", ""));
        assert_eq!(result, Some(400));
    }
}
