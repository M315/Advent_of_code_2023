use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum Module {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunct(Conjunct),
}

impl Module {
    fn new(input: &str) -> Self {
        match input.chars().next() {
            Some('%') => Self::FlipFlop(FlipFlop::new(input)),
            Some('&') => Self::Conjunct(Conjunct::new(input)),
            _ => Self::Broadcaster(Broadcaster::new(input)),
        }
    }

    fn name(&self) -> String {
        match self {
            Self::Broadcaster(module) => module.name.clone(),
            Self::FlipFlop(module) => module.name.clone(),
            Self::Conjunct(module) => module.name.clone(),
        }
    }

    fn destinations(&self) -> &Vec<String> {
        match self {
            Self::Broadcaster(module) => &module.destinations,
            Self::FlipFlop(module) => &module.destinations,
            Self::Conjunct(module) => &module.destinations,
        }
    }

    fn apply(&mut self, pulse: Pulse, sender: String, iteration: u64) -> VecDeque<(String, Pulse, String)> {
        match self {
            Self::Broadcaster(module) => module.apply(pulse),
            Self::FlipFlop(module) => module.apply(pulse),
            Self::Conjunct(module) => module.apply(pulse, sender, iteration),
        }
    }

    fn memory(&self) -> Option<HashMap<String, Pulse>> {
        match self {
            Self::Conjunct(module) => Some(module.memory.clone()),
            _ => None,
        }
    }

    fn cycle(&self) -> Option<u64> {
        match self {
            Self::Conjunct(module) => module.cycle,
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Broadcaster {
    name: String,
    destinations: Vec<String>,
}

impl Broadcaster {
    fn new(input: &str) -> Self {
        let (name, destinations) = input.split_once(" -> ").unwrap();
        Self {
            name: name.to_string(),
            destinations: destinations.split(",").map(|s| s.trim().to_string()).collect(),
        }
    }

    fn apply(&mut self, pulse: Pulse) -> VecDeque<(String, Pulse, String)> {
        self.send(pulse)
    }

    fn send(&mut self, pulse: Pulse) -> VecDeque<(String, Pulse, String)> {
        self.destinations.iter()
            .map(|s| (s.clone(), pulse, self.name.clone()))
            .collect()
    }
}

#[derive(Debug, Clone)]
struct FlipFlop {
    name: String,
    status: bool,
    destinations: Vec<String>,
}

impl FlipFlop {
    fn new(input: &str) -> Self {
        let (name, destinations) = input.split_once(" -> ").unwrap();
        Self {
            name: name.strip_prefix("%").unwrap().to_string(),
            status: false,
            destinations: destinations.split(",").map(|s| s.trim().to_string()).collect(),
        }
    }

    fn apply(&mut self, pulse: Pulse) -> VecDeque<(String, Pulse, String)> {
        if pulse == Pulse::High { return VecDeque::new(); }
        match self.status {
            true => {
                self.status = false;
                self.send(Pulse::Low)
            },
            false => {
                self.status = true;
                self.send(Pulse::High)
            }
        }
    }

    fn send(&mut self, pulse: Pulse) -> VecDeque<(String, Pulse, String)> {
        self.destinations.iter()
            .map(|s| (s.clone(), pulse, self.name.clone()))
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Conjunct {
    name: String,
    memory: HashMap<String, Pulse>,
    destinations: Vec<String>,
    pulses: Result<u64, u64>,
    cycle: Option<u64>,
}

impl Conjunct {
    fn new(input: &str) -> Self {
        let (name, destinations) = input.split_once(" -> ").unwrap();
        Self {
            name: name.strip_prefix("&").unwrap().to_string(),
            memory: HashMap::new(),
            destinations: destinations.split(",").map(|s| s.trim().to_string()).collect(),
            pulses: Err(0),
            cycle: None,
        }
    }

    fn apply(&mut self, pulse: Pulse, sender: String, iteration: u64) -> VecDeque<(String, Pulse, String)> {
        self.memory.entry(sender).and_modify(|p| *p = pulse);
        match self.memory.values().all(|&p| p == Pulse::High) {
            true => { self.send(Pulse::Low, iteration) },
            false => { self.send(Pulse::High, iteration) },
        }
    }

    fn send(&mut self, pulse: Pulse, iteration: u64) -> VecDeque<(String, Pulse, String)> {
        match pulse {
            Pulse::Low => {
                self.pulses = match self.pulses {
                    Ok(i) => Err(i),
                    Err(i) => Err(i),
                };
            },
            Pulse::High => {
                self.pulses = match self.pulses {
                    Ok(_) => Ok(iteration),
                    Err(last_iteration) => {
                        self.cycle = Some(self.cycle.unwrap_or(0).max(iteration - last_iteration));
                        Ok(iteration)
                    },
                };
            },
        };
        self.destinations.iter()
            .map(|s| (s.clone(), pulse, self.name.clone()))
            .collect()
    }
}

fn parse(input: &str) -> HashMap<String, Module> {
    input.lines()
        .map(|line| Module::new(line))
        .map(|module| (module.name(), module))
        .collect()
}

fn init_conjuncts(modules: &mut HashMap<String, Module>) {
    for module in modules.clone().values() {
        for next_module in module.destinations() {
            if let Some(Module::Conjunct(conj)) = modules.get_mut(next_module) {
                conj.memory.insert(module.name(), Pulse::Low);
            }
        }
    }
}

fn pulse(modules: &mut HashMap<String, Module>, iteration: u64) -> (u64, u64) {
    let mut low: u64 = 0;
    let mut high: u64 = 0;

    let mut q: VecDeque<(String, Pulse, String)> = VecDeque::new();
    q.push_back((String::from("broadcaster"), Pulse::Low, String::from("button")));

    while let Some((module_name, pulse, sender)) = q.pop_front() {
        match pulse {
            Pulse::High => high += 1,
            Pulse::Low => low += 1,
        }
        if let Some(module) = modules.get_mut(&module_name) {
            q.append(&mut module.apply(pulse, sender, iteration));
        }
    }
     
    (low, high)
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

pub fn part_one(input: &str) -> Option<u64> {
    let mut modules = parse(input);
    init_conjuncts(&mut modules);
    let mut pulses: (u64, u64) = (0, 0);
    for k in 0..1000 {
        let round = pulse(&mut modules, k);
        pulses = (pulses.0 + round.0, pulses.1 + round.1);
    }
    Some(pulses.0 * pulses.1)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut modules = parse(input);
    init_conjuncts(&mut modules);
    for k in 1..10_000 {
        pulse(&mut modules, k);
    }
    modules.get("lx")
        .unwrap()
        .memory()
        .unwrap()
        .keys()
        .map(|module_name| modules.get(module_name).unwrap().cycle())
        .fold(Some(1), |acc, cycle| lcm(acc, cycle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }
}
