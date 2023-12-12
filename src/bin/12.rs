use std::collections::HashMap;

advent_of_code::solution!(12);

#[derive(Debug)]
struct Spring {
    data: Vec<char>,
    groups: Vec<usize>,
}

impl Spring {
    fn new(input: &str) -> Self {
        let (data, groups) = input.split_once(" ").unwrap();
        Spring {
            data: data.chars().collect(),
            groups: groups.split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>().into_iter().rev().collect(),
        }
    }

    fn extend(&self, n: usize) -> Self {
        let mut new_data: Vec<char> = self.data.to_vec();
        new_data.push('?');
        new_data = new_data.repeat(n);
        new_data.pop();
        Self {
            data: new_data,
            groups: self.groups.repeat(n),
        }
    }

    fn valid(&self, pos: usize, strike: usize) -> Option<u64> {
        // No more groups
        if self.groups.len() == 0 {
            if self.data[pos..].iter().all(|&c| c == '.' || c == '?') {
                return Some(1);
            } else {
                return Some(0);
            }
        }

        // End of string
        if pos == self.data.len() {
            return match self.groups.len() {
                0 => Some(1),
                _ => {
                    if self.groups.len() == 1 && self.groups[0] == strike {
                        return Some(1);
                    } else {
                        return Some(0);
                    }
                },
            };
        }

        // No more ?
        if !self.data[pos..].iter().any(|&c| c == '?') {
            let group_slice = self.data[pos - strike..].iter().collect::<String>();
            let broken: Vec<&str> = group_slice.split(".").filter(|&s| !s.is_empty()).collect();
            if broken.len() != self.groups.len() { return Some(0); }
            let condition = broken.into_iter()
                .zip(self.groups.iter().rev())
                .fold(true, |acc, (broken, &length)| acc && broken.len() == length);
            if condition {
                return Some(1);
            } else {
                return Some(0);
            }
        }

        None
    }

    // Need better pruning
    fn prune(&self, pos: usize, strike: usize) -> bool {
        if strike > *self.groups.last().unwrap() { return true; }
        false
    }

    fn ways(&self, pos: usize, strike: usize, memo: &mut HashMap<(String, Vec<usize>, usize), u64>) -> u64 {
        // Valid
        match self.valid(pos, strike) {
            Some(result) => { return result; },
            None => {},
        }

        // Seen
        if memo.contains_key(&(self.data[pos..].iter().collect::<String>(), self.groups.to_vec(), strike)) {
            return *memo.get(&(self.data[pos..].iter().collect::<String>(), self.groups.to_vec(), strike)).unwrap();
        }

        // Prune
        if self.prune(pos, strike) { return 0; }

        let combinations = match self.data[pos] {
            '#' => { self.ways(pos + 1, strike + 1, memo) },
            '.' => {
                if strike != 0 && strike != *self.groups.last().unwrap() { return 0; }
                match strike {
                    0 => self.ways(pos + 1, 0, memo),
                    _ => {
                        let updated_groups = self.groups[..self.groups.len() - 1].to_vec();
                        (Self{ data: self.data.to_vec(), groups: updated_groups }).ways(pos + 1, 0, memo)
                    }
                }
            },
            '?' => {
                let mut updated_data = self.data.to_vec();
                updated_data[pos] = '#';
                let broken: u64 = (Self{ data: updated_data.to_vec(), groups: self.groups.to_vec() }).ways(pos + 1, strike + 1, memo);

                updated_data[pos] = '.';
                let save: u64 = (Self{ data: updated_data, groups: self.groups.to_vec() }).ways(pos, strike, memo);

                save + broken
            },
            _ => panic!("Unexpected character"),
        };

        memo.insert((self.data[pos..].iter().collect::<String>(), self.groups.to_vec(), strike), combinations);
        combinations
    }
}

fn parse(input: &str) -> Vec<Spring> {
    input.lines()
        .map(|line| Spring::new(line))
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    parse(input)
        .into_iter()
        .fold(Some(0), |acc, spring| Some(acc.unwrap() + spring.ways(0, 0,&mut HashMap::new())))
}

pub fn part_two(input: &str) -> Option<u64> {
    parse(input).iter_mut() 
        .fold(Some(0), |acc, spring| {
            Some(acc.unwrap() + spring.extend(5).ways(0, 0, &mut HashMap::new()))//back_combinations.max(forward_combinations))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
