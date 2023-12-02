use std::collections::HashMap;

advent_of_code::solution!(2);

fn generate_round(round: &str) -> HashMap<String, u32> {
    round.split(",")
        .map(|draw| draw.split_once(" ").unwrap().1.split_once(" ").unwrap())
        .fold(HashMap::new(), |mut acc, (n, color)| {
            acc.insert(color.to_string(), n.trim().parse::<u32>().unwrap());
            acc
        })
}

fn parse(input: &str) -> Vec<Vec<HashMap<String, u32>>> {
    input.lines()
        .map(|line| line.split_once(":").unwrap().1)
        .map(|line| line.split(";").map(|round| generate_round(round)).collect())
        .collect()
}

fn valid_game(game: &Vec<HashMap<String, u32>>, bag: &HashMap<String, u32>) -> bool {
    for round in game {
        for (color, m) in bag.iter() {
            if let Some(n) = round.get(color) {
                if n > m { return false; }
            }
        }
    }
    true
}

fn power_set(game: &Vec<HashMap<String, u32>>) -> u64 {
    game.iter()
        .fold(HashMap::new(), |mut acc, round| {
            for (color, n) in round.iter() {
                acc.entry(color.to_string()).and_modify(|m| *m = *n.max(m)).or_insert(*n);
            }
            acc
        })
        .into_values()
        .fold(1, |acc, m| acc * m as u64)
}

pub fn part_one(input: &str) -> Option<u32> {
    let bag: HashMap<String, u32> = HashMap::from([
        (String::from("red"), 12),
        (String::from("green"), 13),
        (String::from("blue"), 14),
    ]);
    let games = parse(input);
    games.into_iter()
        .enumerate()
        .filter(|(_, game)| valid_game(&game, &bag))
        .fold(Some(0), |acc, (i, _)| Some(acc.unwrap() + i as u32 + 1))
}

pub fn part_two(input: &str) -> Option<u64> {
    let games = parse(input);
    games.into_iter()
        .fold(Some(0), |acc, game| Some(acc.unwrap() + power_set(&game)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
