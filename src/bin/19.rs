use std::cmp::Ordering;
use std::collections::HashMap;

advent_of_code::solution!(19);

#[derive(Debug)]
struct Rule {
    destination: String,
    order: Option<Ordering>,
    value: Option<u32>,
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

    fn apply(&self, val: u32) -> Option<String> {
        match self.order {
            None => Some(self.destination.clone()),
            Some(Ordering::Less) => { if val < self.value.unwrap() { Some(self.destination.clone()) } else { None } },
            Some(Ordering::Greater) => { if val > self.value.unwrap() { Some(self.destination.clone()) } else { None } },
            Some(Ordering::Equal) => panic!("Invalid comparison"),
        }
    }

    fn next_workflow(&self, pice: &Vec<u32>) -> Option<String> {
        match self.kind {
            None => { return Some(self.destination.clone()); },
            Some('x') => { return self.apply(pice[0]); }
            Some('m') => { return self.apply(pice[1]); }
            Some('a') => { return self.apply(pice[2]); }
            Some('s') => { return self.apply(pice[3]); }
            _ => panic!("Invalid kind"),
        }
    }
}

fn valid_pice(pice: &Vec<u32>, workflows: &HashMap<String, Vec<Rule>>) -> bool {
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

pub fn part_one(input: &str) -> Option<u32> {
    let workflows: HashMap<String, Vec<Rule>> = parse(input.split_once("\n\n").unwrap().0);
    input.split_once("\n\n")
        .unwrap()
        .1
        .lines()
        .fold(None, |mut acc, pice| {
            let pice: Vec<u32> = pice.strip_suffix("}")
                .unwrap()
                .split(",")
                .map(|part| part.split_once("=").unwrap().1.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            if valid_pice(&pice, &workflows) {
                acc = Some(acc.unwrap_or(0) + pice.into_iter().fold(0, |s, n| s + n));
            }
            acc
        })
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
