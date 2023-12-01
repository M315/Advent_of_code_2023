use std::collections::HashMap;
advent_of_code::solution!(1);

fn parse(input: &str) -> Vec<Vec<u32>> {
    
    input.lines()
         .map(|line| line.chars().filter(|c| c.is_digit(10)).map(|d| d.to_digit(10).unwrap()).collect::<Vec<u32>>())
         .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    parse(input).into_iter()
                .fold(Some(0), |acc, v| Some(acc.unwrap() + v[0] * 10 + v.last().unwrap()))
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers: HashMap<&str, &str> = HashMap::from([
        ("one", "o1ne"),
        ("two", "t2wo"),
        ("three", "t3hree"),
        ("four", "f4our"),
        ("five", "f5ive"),
        ("six", "s6ix"),
        ("seven", "s7even"),
        ("eight", "e8ight"),
        ("nine", "n9ine"),
    ]);
    let mut s: String = String::from(input);
    for (key, val) in numbers.into_iter() {
        s = s.replace(key, val);
    }

    parse(s.as_str()).into_iter()
                .fold(Some(0), |acc, v| Some(acc.unwrap() + v[0] * 10 + v.last().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(142));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
