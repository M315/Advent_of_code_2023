use std::collections::HashMap;

advent_of_code::solution!(8);

fn parse(input: &str) -> HashMap<&str, (&str, &str)> {
    input.lines()
        .skip(2)
        .fold(HashMap::new(), |mut acc, line| {
            let key: &str = line.get(0..3).unwrap();
            let left: &str = line.get(7..10).unwrap();
            let right: &str = line.get(12..15).unwrap();
            acc.insert(key, (left, right));
            acc
        })
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a.rem_euclid(b);
        a = t;
    }
    a
}

fn lcm(a: Option<u64>, b: Option<u64>) -> Option<u64> {
    if a.is_none() || b.is_none() { return None; }
    Some((a.unwrap() * b.unwrap()) / gcd(a.unwrap(), b.unwrap()))
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: HashMap<&str, (&str, &str)> = parse(input);

    let mut count: u32 = 0;
    let mut node: &str = "AAA";
    let mut instruction = input.lines().take(1).next().unwrap().chars().cycle();
    while node != "ZZZ" {
        match instruction.next().unwrap() {
            'L' => { node = map.get(node).unwrap().0; },
            'R' => { node = map.get(node).unwrap().1; },
            _ => panic!("Invalid instruction"),
        }
        count += 1;
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map: HashMap<&str, (&str, &str)> = parse(input);

    let mut count: u32 = 0;
    let mut nodes: Vec<&str> = map.keys()
        .filter(|key| key.ends_with("A"))
        .map(|key| *key)
        .collect();
    let mut cycles: Vec<Option<u64>> = vec![None; nodes.len()];
    let mut instruction = input.lines().take(1).next().unwrap().chars().cycle();
    while cycles.iter().filter(|cycle| cycle.is_none()).count() != 0 {
        count += 1;
        let instruction: Option<char> = instruction.next();
        for (i, node) in nodes.iter_mut().enumerate() {
            match instruction {
                Some('L') => { *node = map.get(node).unwrap().0; },
                Some('R') => { *node = map.get(node).unwrap().1; },
                _ => panic!("Invalid instruction"),
            }
            if node.ends_with("Z") && cycles[i].is_none() {
                cycles[i] = Some(count as u64);
            }
        }
    }
    cycles.into_iter()
        .fold(Some(1), |acc, n| lcm(acc, n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
