// Shoelace formula & Pick's theorem

advent_of_code::solution!(18);

fn shoelace_formula(curve: &Vec<(i64, i64)>) -> i64 {
    let mut area: i64 = 0;

    area += curve[0].0 * (curve.last().unwrap().1 - curve[1].1);
    for i in 1..curve.len() - 1 {
        area += curve[i].0 * (curve[i - 1].1 - curve[i + 1].1);
    }
    area += curve.last().unwrap().0 * (curve[curve.len() - 2].1 - curve[0].1);

    area.abs() / 2
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut pos: (i64, i64) = (0, 0);
    let mut perimeter: i64 = 0;
    let curve: Vec<(i64, i64)> = input.lines()
        .map(|line| {
            let curr_pos = pos;
            let parts: Vec<&str> = line.split(" ").collect();
            let length: i64 = parts[1].parse().unwrap();
            perimeter += length;
            match parts[0]  {
                "R" => { pos.1 += length; }
                "L" => { pos.1 -= length; }
                "D" => { pos.0 += length; }
                "U" => { pos.0 -= length; }
                _ => panic!("Invalid direction"),
            }
            curr_pos
        }).collect();
    let area = shoelace_formula(&curve);
    let interior_area = area - (perimeter / 2) + 1;
    Some(perimeter + interior_area)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut pos: (i64, i64) = (0, 0);
    let mut perimeter: i64 = 0;
    let curve: Vec<(i64, i64)> = input.lines()
        .map(|line| {
            let curr_pos = pos;
            let (length, dir): (&str, &str) = line.split(" ")
                .collect::<Vec<&str>>()[2]
                .strip_prefix("(#")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split_at(5);
            let length: i64 = i64::from_str_radix(length, 16).unwrap();
            perimeter += length;
            match dir  {
                "0" => { pos.1 += length; }
                "2" => { pos.1 -= length; }
                "1" => { pos.0 += length; }
                "3" => { pos.0 -= length; }
                _ => panic!("Invalid direction"),
            }
            curr_pos
        }).collect();
    let area = shoelace_formula(&curve);
    let interior_area = area - (perimeter / 2) + 1;
    Some(perimeter + interior_area)
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
        assert_eq!(result, Some(952408144115));
    }
}
