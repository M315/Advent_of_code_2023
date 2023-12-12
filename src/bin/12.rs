use std::time::Instant;

advent_of_code::solution!(12);

#[derive(Debug, Clone)]
struct Spring {
    data: String,
    groups: Vec<usize>,
}

impl Spring {
    fn new(input: &str) -> Self {
        let (data, groups) = input.split_once(" ").unwrap();
        Spring {
            data: data.to_string(),
            groups: groups.split(",").map(|n| n.parse::<usize>().unwrap()).collect(),
        }
    }

    fn extend(&self, n: usize) -> Self {
        let mut new_data: String = (self.data.to_string() + "?").repeat(n);
        new_data.pop();
        Self {
            data: new_data,
            groups: self.groups.repeat(n),
        }
    }

    fn valid(&self) -> bool {
        let broken: Vec<&str> = self.data.split(".").filter(|&s| !s.is_empty()).collect();
        if broken.len() != self.groups.len() { return false; }
        broken.into_iter()
            .zip(self.groups.iter())
            .fold(true, |acc, (broken, &length)| acc && broken.len() == length)
    }

    fn prune(&self) -> bool {
        let maybe_broken: Vec<&str> = self.data.split(".")
            .filter(|&s| !s.is_empty())
            .collect();
        if maybe_broken.iter().filter(|&s| !s.chars().all(|c| c == '?')).count() > self.groups.len() { return true; }
        let mut result: bool = false;
        if self.groups.len() > maybe_broken.len() + self.data.chars().filter(|&c| c == '?').count() { return true; }
        for (s, &length) in maybe_broken.into_iter().zip(self.groups.iter()) {
            if s.chars().all(|c| c == '?') { break; }
            //if *s.as_bytes().first().unwrap() == b'?' { break; }
            result |= s.len() < length || (s.len() > length && s.chars().take(length + 1).all(|c| c == '#'));
            if s.chars().any(|c| c == '?') { break; }
        }
        //println!("{} {:?}", result, self);
        result
    }

    fn ways(&self, memo: &mut Vec<String>) -> u64 {
        match self.data.find('?') {
            None => {
                if self.valid() {
                    //println!("{:?}", self);
                    memo.push(self.data.to_string());
                    1
                } else {
                    0
                }
            },
            Some(pos) => {
                if self.prune() { return 0; }
                let mut new_data: String = self.data.to_string();

                new_data.replace_range(pos..pos + 1, ".");
                let fine: u64 = Self { data: new_data.to_string(), groups: self.groups.to_vec() }.ways(memo);

                new_data.replace_range(pos..pos + 1, "#");
                let broken: u64 = Self { data: new_data.to_string(), groups: self.groups.to_vec() }.ways(memo);

                fine + broken
            }
        }
    }
}

fn parse(input: &str) -> Vec<Spring> {
    input.lines()
        .map(|line| Spring::new(line))
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    parse(input).into_iter().fold(Some(0), |acc, spring| Some(acc.unwrap() + spring.ways(&mut Vec::new())))
}

pub fn part_two(input: &str) -> Option<u64> {
    //let mut memo: Vec<String> = Vec::new();
    //let mut spring: Spring = parse(input)[5].clone();
    //let ans = spring.ways(&mut memo);
    //println!("Original");
    //for sol in memo {
    //    println!("{:?}", sol);
    //}
    //println!("{}", ans);
    //println!("Added");
    //let mut memo_2: Vec<String> = Vec::new();
    //spring.data.insert(0, '?');
    //let ans = spring.ways(&mut memo_2);
    //for sol in memo_2 {
    //    println!("{:?}", sol);
    //}
    //println!("{}", ans);
    //println!("Added_2");
    //let mut memo_2: Vec<String> = Vec::new();
    //spring.data.push('?');
    //let ans = spring.ways(&mut memo_2);
    //for sol in memo_2 {
    //    println!("{:?}", sol);
    //}
    //println!("{}", ans);
    //Some(ans)
    parse(input).iter_mut().enumerate().skip(27).take(1)
        .fold(Some(0), |acc, (i, spring)| {
            let now = Instant::now();
            let mut original_ways: Vec<String> = Vec::new();
            spring.ways(&mut original_ways);

            print!("{}, {:?} ", i, (Spring{ data: spring.data.to_string() + "?" + spring.data.as_str(), groups: spring.groups.repeat(2)}).ways(&mut Vec::new()));

            let mut back_ways: Vec<String> = Vec::new();
            spring.data.push('?');
            spring.ways(&mut back_ways);

            let mut back_combinations: u64 = 0;
            for way in &original_ways {
                let mut valid_combinations: u64 = 0;
                for back_way in &back_ways {
                    if (Spring { data: back_way.to_owned() + way, groups: spring.groups.repeat(2) }).valid() {
                        valid_combinations += 1;
                    }
                }
                back_combinations += valid_combinations.pow(4);
            }
            print!("{} ", back_combinations);


            let mut forward_ways: Vec<String> = Vec::new();
            spring.data.pop();
            spring.data.insert(0, '?');
            spring.ways(&mut forward_ways);

            let mut forward_combinations: u64 = 0;
            for way in &original_ways {
                let mut valid_combinations: u64 = 0;
                for forward_way in &forward_ways {
                    if (Spring { data: way.to_owned() + forward_way, groups: spring.groups.repeat(2) }).valid() {
                        valid_combinations += 1;
                    }
                }
                forward_combinations += valid_combinations.pow(4);
            }
            print!("{}", forward_combinations);
            println!(" -> {:?} {} {} {}", spring, original_ways.len(), back_ways.len(), forward_ways.len());
            println!("{:?}", now.elapsed());

            if forward_combinations as usize == original_ways.len() * forward_ways.len().pow(4)
            || back_combinations as usize == original_ways.len() * back_ways.len().pow(4) {
                println!("THIS ONE\n");
            }


            Some(acc.unwrap() + back_combinations.max(forward_combinations))
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
