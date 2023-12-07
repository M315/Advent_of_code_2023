advent_of_code::solution!(6);

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn winning_ways(&self) -> u64 {
        // Solve h^2 - t*h + d = 0
        // We want the values of h that h^2 - t*h + d < 0
        // We want the values from h- to h+ 
        let s: f64 = ((self.time * self.time - 4 * self.distance) as f64).sqrt().ceil();
        (((self.time as f64 + s) / 2.0).ceil() - ((self.time as f64 - s) / 2.0).floor()) as u64 - 1
    }
}

fn parse(input: &str) -> Vec<Race> {
    let values: Vec<Vec<&str>> = input.lines()
        .map(|line| line.split_once(":").unwrap().1.trim().split(" ").filter(|n| *n != "").collect())
        .collect();
    println!("{:?}", values);
    values[0].iter()
        .zip(values[1].iter())
        .map(|(t, d)| Race { time: t.trim().parse::<u64>().unwrap(), distance: d.trim().parse::<u64>().unwrap()} )
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let races: Vec<Race> = parse(input);
    races.into_iter().fold(Some(1), |acc, race| Some(acc.unwrap() * race.winning_ways()))
}

pub fn part_two(input: &str) -> Option<u64> {
    let values: Vec<&str> = input.lines().map(|line| line.split_once(":").unwrap().1.trim()).collect();
    let race: Race = Race {
        time: values[0].replace(" ", "").parse::<u64>().unwrap(),
        distance: values[1].replace(" ", "").parse::<u64>().unwrap(),
    };
    Some(race.winning_ways())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
