use std::cmp::Ordering;
use std::collections::HashMap;

advent_of_code::solution!(19);

#[derive(Debug)]
struct Rule {
    destination: String,
    order: Option<Ordering>,
    value: Option<u64>,
    kind: Option<char>,
}

impl Rule {
    fn from(input: &str) -> Self {
        if !input.contains(":") { return Self{ destination: input.to_string(), order: None, value: None, kind: None }; }
        let (predicate, destination) = input.split_once(":").unwrap();
        if predicate.contains("<") {
            Self {
                destination: destination.to_string(),
                order: Some(Ordering::Less),
                value: Some(predicate.split_at(2).1.parse().unwrap()),
                kind: predicate.chars().next(),
            }
        } else if predicate.contains(">") {
            Self {
                destination: destination.to_string(),
                order: Some(Ordering::Greater),
                value: Some(predicate.split_at(2).1.parse().unwrap()),
                kind: predicate.chars().next(),
            }
        } else {
            panic!("Invalid rule");
        }
    }

    fn apply(&self, val: u64) -> Option<String> {
        match self.order {
            None => Some(self.destination.clone()),
            Some(Ordering::Less) => { if val < self.value.unwrap() { Some(self.destination.clone()) } else { None } },
            Some(Ordering::Greater) => { if val > self.value.unwrap() { Some(self.destination.clone()) } else { None } },
            Some(Ordering::Equal) => panic!("Invalid comparison"),
        }
    }

    fn next_workflow(&self, pice: &Vec<u64>) -> Option<String> {
        match self.kind {
            None => { return Some(self.destination.clone()); },
            Some('x') => { return self.apply(pice[0]); }
            Some('m') => { return self.apply(pice[1]); }
            Some('a') => { return self.apply(pice[2]); }
            Some('s') => { return self.apply(pice[3]); }
            _ => panic!("Invalid kind"),
        }
    }

    fn next_range(&self, pice_range: &mut PiceRange) -> PiceRange {
        let mut move_on_pice = pice_range.clone();
        move_on_pice.workflow = self.destination.clone();
        match self.kind {
            None => {
                pice_range.x.0 = 4000;
                pice_range.x.1 = 0;
            },
            Some('x') => {
                match self.order {
                    Some(Ordering::Less) => {
                        move_on_pice.x.1 = self.value.unwrap() - 1;
                        pice_range.x.0 = self.value.unwrap();
                    },
                    Some(Ordering::Greater) => {
                        pice_range.x.1 = self.value.unwrap();
                        move_on_pice.x.0 = self.value.unwrap() + 1;
                    },
                    _ => panic!("Invalid comparison"),
                }
            },
            Some('m') => {
                match self.order {
                    Some(Ordering::Less) => {
                        move_on_pice.m.1 = self.value.unwrap() - 1;
                        pice_range.m.0 = self.value.unwrap();
                    },
                    Some(Ordering::Greater) => {
                        pice_range.m.1 = self.value.unwrap();
                        move_on_pice.m.0 = self.value.unwrap() + 1;
                    },
                    _ => panic!("Invalid comparison"),
                }
            },
            Some('a') => {
                match self.order {
                    Some(Ordering::Less) => {
                        move_on_pice.a.1 = self.value.unwrap() - 1;
                        pice_range.a.0 = self.value.unwrap();
                    },
                    Some(Ordering::Greater) => {
                        pice_range.a.1 = self.value.unwrap();
                        move_on_pice.a.0 = self.value.unwrap() + 1;
                    },
                    _ => panic!("Invalid comparison"),
                }
            },
            Some('s') => {
                match self.order {
                    Some(Ordering::Less) => {
                        move_on_pice.s.1 = self.value.unwrap() - 1;
                        pice_range.s.0 = self.value.unwrap();
                    },
                    Some(Ordering::Greater) => {
                        pice_range.s.1 = self.value.unwrap();
                        move_on_pice.s.0 = self.value.unwrap() + 1;
                    },
                    _ => panic!("Invalid comparison"),
                }
            },
            _ => panic!("Invalid kind"),
        }

        move_on_pice
    }
}

#[derive(Debug, Clone)]
struct PiceRange {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
    workflow: String,
}

impl PiceRange {
    fn apply_workflow(&mut self, workflows: &HashMap<String, Vec<Rule>>) -> Vec<PiceRange> {
        let mut new_ranges: Vec<PiceRange> = Vec::new();
        for rule in workflows.get(&self.workflow).unwrap() {
            if !self.valid() { break; }
            let next_range = rule.next_range(self);
            if next_range.valid() { new_ranges.push(next_range); }
        }
        new_ranges
    }

    fn valid(&self) -> bool {
        if self.x.0 > self.x.1 || self.m.0 > self.m.1 || self.a.0 > self.a.1 || self.s.0 > self.s.1 {
            return false;
        } 
        true
    }

    fn combinations(&self) -> u64 {
        (self.x.1 - self.x.0 + 1) * (self.m.1 - self.m.0 + 1) * (self.a.1 - self.a.0 + 1) * (self.s.1 - self.s.0 + 1)
    }
}

fn valid_pice(pice: &Vec<u64>, workflows: &HashMap<String, Vec<Rule>>) -> bool {
    let mut workflow: String = String::from("in");

    loop {
        if workflow == "A" { return true; }
        if workflow == "R" { return false; }

        for rule in workflows.get(&workflow).unwrap() {
            if let Some(next_workflow) = rule.next_workflow(pice) {
                workflow = next_workflow;
                break;
            }
        }
    }
}

fn parse(input: &str) -> HashMap<String, Vec<Rule>> {
    input.lines()
        .map(|line| {
            let (workflow, rules) = line.split_once("{").unwrap();
            let rules: Vec<Rule> = rules.strip_suffix("}").unwrap().split(",").map(|rule| Rule::from(rule)).collect();
            (workflow.to_string(), rules)
        }).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let workflows: HashMap<String, Vec<Rule>> = parse(input.split_once("\n\n").unwrap().0);
    input.split_once("\n\n")
        .unwrap()
        .1
        .lines()
        .fold(None, |mut acc, pice| {
            let pice: Vec<u64> = pice.strip_suffix("}")
                .unwrap()
                .split(",")
                .map(|part| part.split_once("=").unwrap().1.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            if valid_pice(&pice, &workflows) {
                acc = Some(acc.unwrap_or(0) + pice.into_iter().fold(0, |s, n| s + n));
            }
            acc
        })
}

pub fn part_two(input: &str) -> Option<u64> {
    let workflows: HashMap<String, Vec<Rule>> = parse(input.split_once("\n\n").unwrap().0);
    let mut pice_ranges: Vec<PiceRange> = vec![PiceRange { x: (1, 4000), m: (1, 4000), a: (1, 4000), s: (1, 4000), workflow: String::from("in")}];
    let mut accepted_pices: Vec<PiceRange> = Vec::new();
    
    while let Some(pice) = pice_ranges.pop().as_mut() {
        let new_ranges = pice.apply_workflow(&workflows);
        for range in new_ranges {
            match range.workflow.as_str() {
                "A" => accepted_pices.push(range),
                "R" => {},
                _ => pice_ranges.push(range),
            }
        }
    }
    accepted_pices.into_iter()
        .fold(None, |acc, pice_range| Some(acc.unwrap_or(0) + pice_range.combinations()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY).replace("\r", ""));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY).replace("\r", ""));
        assert_eq!(result, Some(167409079868000));
    }
}
