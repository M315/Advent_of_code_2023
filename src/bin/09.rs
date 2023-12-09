advent_of_code::solution!(9);

fn parse(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(|line| line.split_whitespace()
            .map(|n| n.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>())
        .collect()
}

fn predict(hist: &Vec<i32>) -> i32 {
    let mut last_values: Vec<i32> = Vec::with_capacity(hist.len() + 1);
    let mut curr: Vec<i32> = hist.to_vec();
    while curr.iter().filter(|&n| *n == 0).count() != curr.len() {
        last_values.push(*curr.last().unwrap());
        curr = curr.iter()
            .skip(1)
            .zip(curr.iter())
            .map(|(a, b)| a - b)
            .collect();
    }
    last_values.into_iter().rev().fold(0, |acc, n| n + acc)
}

fn predict_backwards(hist: &Vec<i32>) -> i32 {
    let mut frst_values: Vec<i32> = Vec::with_capacity(hist.len() + 1);
    let mut curr: Vec<i32> = hist.to_vec();
    while curr.iter().filter(|&n| *n == 0).count() != curr.len() {
        frst_values.push(*curr.first().unwrap());
        curr = curr.iter()
            .skip(1)
            .zip(curr.iter())
            .map(|(a, b)| a - b)
            .collect();
    }
    frst_values.into_iter().rev().fold(0, |acc, n| n - acc)
}

pub fn part_one(input: &str) -> Option<i32> {
    let histories: Vec<Vec<i32>> = parse(input);
    histories.into_iter()
        .fold(Some(0), |acc, hist| Some(acc.unwrap() + predict(&hist)))
}

pub fn part_two(input: &str) -> Option<i32> {
    let histories: Vec<Vec<i32>> = parse(input);
    histories.into_iter()
        .fold(Some(0), |acc, hist| Some(acc.unwrap() + predict_backwards(&hist)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
