advent_of_code::solution!(5);

#[derive(Debug)]
struct Almanac {
    source: String,
    destination: String,
    ranges: Vec<Vec<u64>>,
}

impl Almanac {
    fn update_stage(&self, stage: &Vec<u64>) -> Vec<u64> {
        let mut result: Vec<u64> = Vec::new();
        for &val in stage {
            let mut found: bool = false;
            for range in &self.ranges {
                if val >= range[1] && val < range[1] + range[2] {
                    result.push(range[0] + (val - range[1]));
                    found = true;
                    break;
                }
            }
            if !found { result.push(val); }
        }
        result
    }
}

fn get_almanac(block: &str) -> Almanac {
    let info: (&str, &str) = block.split_once("\n").unwrap().0
        .split_once(" ").unwrap().0
        .split_once("-to-").unwrap();
    let ranges: Vec<Vec<u64>> = block.lines()
        .skip(1)
        .map(|line| line.split(" ").map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>())
        .collect();
    Almanac {
        source: info.0.to_owned(),
        destination: info.1.to_owned(),
        ranges: ranges,
    }
}

fn parse(input: &str) -> Vec<Almanac> {
    input.split("\n\n")
        .skip(1)
        .map(|block| get_almanac(block))
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut stage: String = String::from("seed");
    let mut seeds: Vec<u64> = input.split_once("\n").unwrap().0
        .split_once(": ").unwrap().1
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    let maps: Vec<Almanac> = parse(input);
    while stage != "location" {
        for almanac in &maps {
            if almanac.source == stage {
                stage = almanac.destination.to_owned();
                seeds = almanac.update_stage(&seeds);
                break;
            }
        }
    }
    seeds.into_iter().min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut stage: String = String::from("seed");
    let seed_ranges: Vec<u64> = input.split_once("\n").unwrap().0
        .split_once(": ").unwrap().1
        .split(" ")
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    let mut seeds: Vec<u64> = Vec::new();
    for range in seed_ranges.chunks(2) {
        for k in 0..range[1] {
            seeds.push(range[0] + k);
        }
    }
    let maps: Vec<Almanac> = parse(input);
    while stage != "location" {
        println!("{:?} {}", stage, seeds.len());
        for almanac in &maps {
            if almanac.source == stage {
                stage = almanac.destination.to_owned();
                seeds = almanac.update_stage(&seeds);
                break;
            }
        }
    }
    seeds.into_iter().min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
