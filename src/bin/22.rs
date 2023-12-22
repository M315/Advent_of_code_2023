use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(22);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick {
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize),
    top: HashSet<usize>,
    bot: HashSet<usize>,
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.z.cmp(&other.z)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Brick {
    fn new(input: &str) -> Self {
        let values: Vec<Vec<usize>> = input.split("~")
            .take(2)
            .map(|brick| brick.split(",")
                .map(|d| d.parse::<usize>().unwrap())
                .collect())
            .collect();
        
        Self {
            x: (values[0][0], values[1][0]),
            y: (values[0][1], values[1][1]),
            z: (values[0][2], values[1][2]),
            top: HashSet::new(),
            bot: HashSet::new(),
        }
    }

    fn pile_into(&mut self, bottom_brick: usize) {
        self.bot.insert(bottom_brick);
    }

    fn pile_onto(&mut self, top_brick: usize) {
        self.top.insert(top_brick);
    }

    fn intersect(&self, other: &Self) -> bool {
        intersection(self.x, other.x) && intersection(self.y, other.y)
    }

    fn destroyable(&self, v: &Vec<Self>) -> bool {
        self.top.iter()
            .fold(true, |valid, &brick| valid & (v[brick].bot.len() > 1))
    }

    fn destruction(&self, idx: usize, bricks: &Vec<Self>) -> u32 {
        let mut falling: VecDeque<usize> = self.top.iter()
            .filter_map(|&n| match bricks[n].bot.len() == 1 {
                true => Some(n),
                false => None,
            })
            .collect();
        let mut fallen: Vec<bool> = vec![false; bricks.len()];
        for &i in falling.iter() { fallen[i] = true; }
        fallen[idx] = true;

        while let Some(brick) = falling.pop_front() {
            for &i in bricks[brick].top.iter() {
                if bricks[i].bot.iter().all(|&j| fallen[j]) && !fallen[i] {
                    fallen[i] = true;
                    falling.push_back(i);
                }
            }
        }

        fallen.into_iter().filter(|&b| b).count() as u32 - 1
    }
}

fn intersection(a: (usize, usize), b: (usize, usize)) -> bool {
    if a.0 > b.0 { return intersection(b, a); }
    a.1 >= b.0
}


fn parse(input: &str) -> Vec<Brick> {
    input.lines()
        .map(|line| Brick::new(line))
        .collect()
}

fn drop(bricks: &mut Vec<Brick>) {
    bricks.sort();
    for i in 0..bricks.len() {
        let mut max_z: usize = 1;
        for j in 0..i {
            if bricks[i].intersect(&bricks[j]) {
                max_z = max_z.max(bricks[j].z.1 + 1);
            }
        }
        bricks[i].z.1 -= bricks[i].z.0 - max_z;
        bricks[i].z.0 = max_z;
    }
}

fn pile(bricks: &mut Vec<Brick>) {
    bricks.sort();
    for i in 0..bricks.len() {
        for j in 0..i {
            if bricks[i].intersect(&bricks[j]) && bricks[i].z.0 == bricks[j].z.1 + 1 {
                bricks[i].pile_into(j);
                bricks[j].pile_onto(i);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut bricks = parse(input);
    drop(&mut bricks);
    pile(&mut bricks);
    Some(bricks.iter()
        .filter(|brick| brick.destroyable(&bricks))
        .count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bricks = parse(input);
    drop(&mut bricks);
    pile(&mut bricks);
    Some(bricks.iter()
        .enumerate()
        .fold(0, |acc, (i, brick)| acc + brick.destruction(i, &bricks)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
