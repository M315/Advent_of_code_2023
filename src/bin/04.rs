use std::collections::HashSet;

advent_of_code::solution!(4);

fn parse_numbers(numbers: &str) -> Vec<u32> {
    numbers.trim()
        .split(" ")
        .filter_map(|num| num.trim().parse::<u32>().ok())
        .collect::<Vec<u32>>()
}

fn parse(input: &str) -> Vec<Vec<Vec<u32>>> {
    input.lines()
        .map(|line| line.split_once(":").unwrap().1.split("|")
            .map(|nums| parse_numbers(nums))
            .collect::<Vec<Vec<u32>>>())
        .collect()
}

fn score(card: &Vec<Vec<u32>>) -> usize {
    let winning: HashSet<u32> = HashSet::from_iter(card[0].clone().into_iter());
    let mine: HashSet<u32> = HashSet::from_iter(card[1].clone().into_iter());
    winning.intersection(&mine).count()
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards: Vec<Vec<Vec<u32>>> = parse(input);
    cards.into_iter()
        .fold(None, |acc, card| match score(&card) {
            0 => Some(acc.unwrap_or(0)),
            n => Some(acc.unwrap_or(0) + (1 << (n - 1))),
        })
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards: Vec<Vec<Vec<u32>>> = parse(input);
    let mut collection: Vec<u32> = vec![1; cards.len()];
    for (i, card) in cards.into_iter().enumerate() {
        for k in i + 1..collection.len().min(i + 1 + score(&card)) {
            collection[k] += collection[i];
        }
    }
    Some(collection.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
