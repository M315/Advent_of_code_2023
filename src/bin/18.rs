advent_of_code::solution!(18);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn from(input: &str) -> Self {
        match input {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Invalid direction"),
        }
    }
}

struct Step {
    dir: Direction,
    steps: usize,
    color: String,
}

impl Step {
    fn from(input: &str) -> Self {
        let parts: Vec<&str> = input.split(" ").collect();
        Self {
            dir: Direction::from(parts[0]),
            steps: parts[1].parse().unwrap(),
            color: parts[2].to_string(),
        }
    }
}

fn dig(start: (usize, usize), plan: &Vec<Step>, grid: &mut Vec<Vec<bool>>) {
    grid[start.0][start.1] = true;
    let mut pos: (usize, usize) = start;
    for step in plan {
        for _ in 0..step.steps {
            match step.dir {
                Direction::Up => { pos.0 -= 1; }
                Direction::Down => { pos.0 += 1; }
                Direction::Left => { pos.1 -= 1; }
                Direction::Right => { pos.1 += 1; }
            };
            grid[pos.0][pos.1] = true;
        }
    }
}

fn out_size(grid: &Vec<Vec<bool>>) -> u32 {
    let mut seen: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    let mut q: Vec<(usize, usize)> = vec![(0, 0)];
    let mut count = 0;
    seen[0][0] = true;

    while let Some((i, j)) = q.pop() {
        count += 1;
        if i > 0 && !grid[i - 1][j] && !seen[i - 1][j] {
            seen[i - 1][j] = true;
            q.push((i - 1, j));
        }
        if i < grid.len() - 1 && !grid[i + 1][j] && !seen[i + 1][j] {
            seen[i + 1][j] = true;
            q.push((i + 1, j));
        }
        if j > 0 && !grid[i][j - 1] && !seen[i][j - 1] {
            seen[i][j - 1] = true;
            q.push((i, j - 1));
        }
        if j < grid[0].len() - 1 && !grid[i][j + 1] && !seen[i][j + 1] {
            seen[i][j + 1] = true;
            q.push((i, j + 1));
        }
    }

    for row in &seen {
        println!("{:?}", row.iter().map(|b| match b {
            true => '#',
            false => '.',
        }).collect::<String>());
    }
    
    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let plan: Vec<Step> = input.lines().map(|line| Step::from(line)).collect();
    // Horizontal limits
    let mut pos: i32 = 0;
    let h_limits: (i32, i32) = plan.iter()
        .fold((0, 0), |acc, state| {
            match state.dir {
                Direction::Left => { pos -= state.steps as i32; },
                Direction::Right => { pos += state.steps as i32; },
                _ => {},
            };
            (acc.0.min(pos), acc.1.max(pos))
        });
    println!("{:?}", h_limits);

    // Vertical limits
    let mut pos: i32 = 0;
    let v_limits: (i32, i32) = plan.iter()
        .fold((0, 0), |acc, state| {
            match state.dir {
                Direction::Up => { pos -= state.steps as i32; },
                Direction::Down => { pos += state.steps as i32; },
                _ => {},
            };
            (acc.0.min(pos), acc.1.max(pos))
        });
    println!("{:?}", v_limits);

    let mut grid: Vec<Vec<bool>> = vec![vec![false; (h_limits.1 - h_limits.0.min(0) + 3) as usize]; (v_limits.1 - v_limits.0.min(0) + 3) as usize];
    dig(((-v_limits.0).max(0) as usize + 1, (-h_limits.0).max(0) as usize + 1), &plan, &mut grid);

    for row in &grid {
        println!("{:?}", row.iter().map(|b| match b {
            true => '#',
            false => '.',
        }).collect::<String>());
    }

    Some((grid.len() * grid[0].len()) as u32 - out_size(&grid))
}

pub fn part_two(input: &str) -> Option<u32> {
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
