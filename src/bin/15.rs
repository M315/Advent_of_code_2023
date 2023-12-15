advent_of_code::solution!(15);

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u32,
}

#[derive(Debug, Clone)]
struct Box {
    lens: Vec<Lens>,
}

impl Lens {
    fn new(input: &str) -> Self {
        let (label, length): (&str, &str) = input.split_once("=").unwrap();
        Self {
            label: label.to_string(),
            focal_length: length.parse::<u32>().unwrap(),
        }
    }
}

impl Box {
    fn new() -> Self {
        Self { lens: Vec::new() }
    }

    fn push(&mut self, new_len: Lens) {
        let mut updated: bool = false;
        for len in self.lens.iter_mut() {
            if new_len.label == len.label {
                len.focal_length = new_len.focal_length;
                updated = true;
            }
        }
        if !updated {
            self.lens.push(new_len);
        }
    }

    fn remove(&mut self, label: &str) {
        let mut pos: usize = 0;
        for len in self.lens.iter() {
            if len.label == *label { break; }
            pos += 1;
        }
        if pos < self.lens.len() {
            self.lens.remove(pos);
        }
    }

    fn value(&self, idx: u32) -> Option<u32> {
        self.lens.iter()
            .enumerate()
            .fold(None, |acc, (k, len)| Some(acc.unwrap_or(0) + idx * (k as u32 + 1) * len.focal_length))
    }
}

fn hash_value(s: &str) -> u32 {
    s.chars()
        .fold(0, |acc, c| ((acc + c as u32) * 17).rem_euclid(256))
}

pub fn part_one(input: &str) -> Option<u32> {
    input.split(",")
        .fold(None, |acc, s| Some(acc.unwrap_or(0) + hash_value(s)))
}

pub fn part_two(input: &str) -> Option<u32> {
    input.split(",")
        .fold(vec![Box::new(); 256], |mut acc, s| {
            if s.ends_with("-") {
                let label = s.strip_suffix("-").unwrap();
                acc[hash_value(label) as usize].remove(label);
            } else {
                let len: Lens = Lens::new(s);
                acc[hash_value(&len.label) as usize].push(len);
            }
            acc
        })
        .into_iter()
        .enumerate()
        .fold(None, |acc, (i, bx)| Some(acc.unwrap_or(0) + bx.value(i as u32 + 1).unwrap_or(0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
