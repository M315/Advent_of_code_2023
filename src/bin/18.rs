use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

advent_of_code::solution!(18);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Orient {
    Horizontal,
    Vertical,
}

impl Ord for Orient {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Orient::Horizontal, Orient::Horizontal) => Ordering::Equal,
            (Orient::Horizontal, Orient::Vertical) => Ordering::Less,
            (Orient::Vertical, Orient::Horizontal) => Ordering::Greater,
            (Orient::Vertical, Orient::Vertical) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Orient {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Step {
    start: (i64, i64),
    end: (i64, i64),
    orientation: Orient,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.start.cmp(&self.start)
            .then_with(|| other.orientation.cmp(&self.orientation))
            .then_with(|| self.end.cmp(&other.end))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Step {
    fn from(input: &str, pos: &mut (i64, i64)) -> Self {
        let parts: Vec<&str> = input.split(" ").collect();
        let steps: i64 = parts[1].parse().unwrap();
        let mut start: (i64, i64) = pos.clone();
        match parts[0] {
            "D" => { pos.0 += steps; },
            "U" => { pos.0 -= steps; start.0 -= steps; },
            "R" => { pos.1 += steps; },
            "L" => { pos.1 -= steps; start.1 -= steps; },
            _ => panic!("Invalid direction"),
        }
        match parts[0] {
            "R" | "L" => Self { start, orientation: Orient::Horizontal, end: (start.0, start.1 + steps) },
            "U" | "D" => Self { start, orientation: Orient::Vertical, end: (start.0 + steps, start.1) },
            _ => panic!("Invalid orientation"),
        }
    }

    fn length(&self) -> i64 {
        self.end.1 - self.start.1
    }

    fn height(&self) -> i64 {
        self.end.0 - self.start.0
    }

    fn intersect(&self, other: &Self) -> bool {
        if self.start.0 != other.start.0 { return false; }
        if self.start.1 > other.start.1 { return other.intersect(self); }
        if self.end.1 < other.start.1 { return false; }
        true
    }

    fn extend(&mut self, other: &Self) {
        self.start.1 = self.start.1.min(other.start.1);
        self.end.1 = self.end.1.max(other.end.1);
    }

    fn shrink_right(&self, other: &Self) -> Self {
        let mut shrinked: Step = other.clone();
        if other.end.1 < self.end.1 {
            shrinked.start.1 = other.end.1;
            shrinked.end.1 = self.end.1;
        } else {
            shrinked.start.1 = self.end.1;
        }
        shrinked
    }

    fn shrink_left(&self, other: &Self) -> Self {
        let mut shrinked: Step = self.clone();
        shrinked.end.1 = other.start.1;
        shrinked
    }

    fn diff_right(&self, other: &Self) -> Self {
        let mut difference = self.clone();
        difference.end.1 = other.start.1;
        difference
    }

    fn diff_left(&self, other: &Self) -> Self {
        let mut difference = self.clone();
        difference.start.1 = other.end.1;
        difference
    }
}

fn size_hole(plan: Vec<Step>) -> u64 {
    let mut vertical_walls: HashMap<(i64, i64), Step> = HashMap::new();
    let mut q: BinaryHeap<Step> = BinaryHeap::new();
    let mut cubes: u64 = 0;
    let mut stripes: HashSet<Step> = HashSet::new();

    for step in plan {
        match step.orientation {
            Orient::Horizontal => { q.push(step); },
            Orient::Vertical => { vertical_walls.insert(step.start, step); },
        }
    }


    while !q.is_empty() {
        println!("# {}", cubes);
        let mut step: Step = q.pop().unwrap();
        println!("{:?}", step);
        while let Some(modifier) = q.pop() {
            if !step.intersect(&modifier) { q.push(modifier); break; }
            println!("\t{:?}", modifier);
            if modifier.start.1 == step.end.1 {
                step.extend(&modifier);
            } else if step.end.1 > modifier.end.1 {
                let left = step.shrink_left(&modifier);
                let right = step.shrink_right(&modifier);
                if left.length() > 0 {
                    //cubes += (step.length() - left.length()) as u64;
                    stripes.insert(step.diff_left(&left));
                    q.push(right);
                    step = left;
                    break;
                } 
                //cubes += (step.length() - right.length()) as u64;
                stripes.insert(step.diff_right(&right));
                step = right;
            } else {
                let left = step.shrink_left(&modifier);
                let right = step.shrink_right(&modifier);
                println!("\t -> {:?} {:?}", left, right);
                if left.length() > 0 && vertical_walls.contains_key(&left.start) && vertical_walls.contains_key(&left.end) {
                    //cubes += (step.length() - left.length()) as u64;
                    stripes.insert(step.diff_left(&left));
                    step = left;
                } else {
                    //cubes += (step.length() - right.length()) as u64;
                    stripes.insert(step.diff_right(&right));
                    step = right;
                }
            }
        }
        println!("\t# {}", cubes);

        if step.length() == 0 { continue; }

        let left: Step = vertical_walls.get(&step.start).unwrap().clone();
        let right: Step = vertical_walls.get(&(step.start.0, &step.start.1 + step.length())).unwrap().clone();
        let height: i64 = (left.height()).min(right.height());

        cubes += (step.length() as u64 + 1) * (height as u64);

        if right.height() > left.height() {
            vertical_walls.insert(
                (step.end.0 + height, step.end.1),
                Step {
                    start: (step.end.0 + height, step.end.1),
                    end: (right.end.0, right.end.1),
                    orientation: Orient::Vertical,
                }
            );
        }

        if left.height() > right.height() {
            vertical_walls.insert(
                (step.start.0 + height, step.start.1),
                Step {
                    start: (step.start.0 + height, step.start.1),
                    end: (left.end.0, left.end.1),
                    orientation: Orient::Vertical,
                }
            );
        }

        q.push(Step {
            start: (step.start.0 + height, step.start.1),
            end: (step.end.0 + height, step.end.1),
            orientation: Orient::Horizontal,
        });
    }
    let mut s = stripes.into_iter().collect::<Vec<Step>>();
    s.sort();
    while !s.is_empty() {
        let mut stripe = s.pop().unwrap();
        println!("{:?}", stripe);
        while let Some(strp) = s.pop() {
            if stripe.intersect(&strp) {
                println!("\t{:?}", strp);
                stripe.extend(&strp);
            } else {
                s.push(strp);
                break;
            }
        }
        println!("-> {:?}", stripe);
        cubes += stripe.length() as u64;
    }
    cubes
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut pos: (i64, i64) = (0, 0);
    let plan: Vec<Step> = input.lines()
        .map(|line| {
            Step::from(line, &mut pos)
        }).collect();
    Some(size_hole(plan))
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
