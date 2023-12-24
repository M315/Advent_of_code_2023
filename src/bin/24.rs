advent_of_code::solution!(24);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hail {
    pos: (i64, i64, i64),
    vel: (i64, i64, i64),
}

impl Hail {
    fn from(input: &str) -> Self {
        let (pos, vel) = input.split_once(" @ ").unwrap();
        let pos: Vec<i64> = pos.split(",").map(|s| s.trim().parse::<i64>().unwrap()).collect();
        let vel: Vec<i64> = vel.split(",").map(|s| s.trim().parse::<i64>().unwrap()).collect();
        Self {
            pos: (pos[0], pos[1], pos[2]),
            vel: (vel[0], vel[1], vel[2]),
        }
    }

    fn planar_intersect(&self, other: &Self) -> Option<((f64, f64), (f64, f64))> {
        let diff = (self.pos.0 - other.pos.0, self.pos.1 - other.pos.1, self.pos.2 - other.pos.2);
        let t: (f64, f64) = (-(diff.0 * other.vel.1 - diff.1 * other.vel.0) as f64, (self.vel.0 * other.vel.1 - self.vel.1 * other.vel.0) as f64);
        let u: (f64, f64) = (-(diff.0 * self.vel.1 - diff.1 * self.vel.0) as f64, (self.vel.0 * other.vel.1 - self.vel.1 * other.vel.0) as f64);

        if t.1 == 0.0 { return None; }
        let intesection: (f64, f64) = (self.pos.0 as f64 + t.0 * self.vel.0 as f64 / t.1, self.pos.1 as f64 + t.0 * self.vel.1 as f64 / t.1);
        Some(((t.0 / t.1, u.0 / u.1), intesection))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let range: (f64, f64) = (200_000_000_000_000.0, 400_000_000_000_000.0);
    let hails: Vec<Hail> = input.lines()
        .map(|line| Hail::from(line))
        .collect();
    hails.iter()
        .enumerate()
        .fold(None, |acc, (i, hail)| {
            Some(hails.iter()
                .skip(i + 1)
                .fold(0, |mut count, inner_hail| {
                    match hail.planar_intersect(inner_hail) {
                        Some((t, pos)) => { if t.0 >= 0.0 && t.1 >= 0.0 && range.0 <= pos.0 && pos.0 <= range.1 && range.0 <= pos.1 && pos.1 <= range.1 { count += 1 } },
                        None => { },
                    };
                    count
                })
                + acc.unwrap_or(0))
        })
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }
}
