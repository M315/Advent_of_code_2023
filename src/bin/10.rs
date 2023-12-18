advent_of_code::solution!(10);

enum Dir {
    Left,
    Right,
    Up,
    Down,
}

fn get_start(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'S' {
                return Some((i, j));
            }
        }
    }
    None
}

fn valid_path(curr_pipe: char, next_pipe: char, dir: Dir) -> bool {
    match dir {
        Dir::Up => {
            (match curr_pipe {
                'S' => true,
                '|' => true,
                'L' => true,
                'J' => true,
                _ => false
            }) && (match next_pipe {
                '|' => true,
                '7' => true,
                'F' => true,
                _ => false
            })
        }
        Dir::Down => {
            (match curr_pipe {
                'S' => true,
                '|' => true,
                '7' => true,
                'F' => true,
                _ => false
            }) && (match next_pipe {
                '|' => true,
                'L' => true,
                'J' => true,
                _ => false
            })
        }
        Dir::Left => {
            (match curr_pipe {
                'S' => true,
                '-' => true,
                'J' => true,
                '7' => true,
                _ => false
            }) && (match next_pipe {
                '-' => true,
                'L' => true,
                'F' => true,
                _ => false
            })
        }
        Dir::Right => {
            (match curr_pipe {
                'S' => true,
                '-' => true,
                'L' => true,
                'F' => true,
                _ => false
            }) && (match next_pipe {
                '-' => true,
                'J' => true,
                '7' => true,
                _ => false
            })
        }
    }
}

fn bfs(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let start: (usize, usize) = get_start(&map).unwrap();
    let mut curr: Vec<(usize, usize)> = vec![start];
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    visited[start.0][start.1] = true;
    
    while !curr.is_empty() {
        let mut next_nodes: Vec<(usize, usize)> = Vec::new();
        for (i, j) in curr {
            if i > 0 && !visited[i - 1][j] && valid_path(map[i][j], map[i - 1][j], Dir::Up) {
                next_nodes.push((i - 1, j));
                visited[i - 1][j] = true;
            }
            if j > 0 && !visited[i][j - 1] && valid_path(map[i][j], map[i][j - 1], Dir::Left) {
                next_nodes.push((i, j - 1));
                visited[i][j - 1] = true;
            }
            if i < map.len() - 1 && !visited[i + 1][j] && valid_path(map[i][j], map[i + 1][j], Dir::Down) {
                next_nodes.push((i + 1, j));
                visited[i + 1][j] = true;
            }
            if j < map[0].len() - 1 && !visited[i][j + 1] && valid_path(map[i][j], map[i][j + 1], Dir::Right) {
                next_nodes.push((i, j + 1));
                visited[i][j + 1] = true;
            }
        }
        curr = next_nodes.to_vec();
    }

    map.into_iter()
        .enumerate()
        .map(|(i, line)| line.iter()
            .enumerate()
            .map(|(j, &c)| match visited[i][j] {
                true => c,
                false => ' ',
            })
            .collect())
        .collect()
}

fn is_interior(i: usize, j: usize, map: &Vec<Vec<char>>) -> bool {
    if map[i][j] != ' ' { return false; }

    let mut intersections: u32 = 0;
    let mut last_open: Option<char> = None;
    for k in i + 1..map.len() {
        match map[k][j] {
            '-' => { intersections += 1; },
            'L' => {
                match last_open {
                    Some('7') => { intersections += 1; last_open = None; },
                    Some('F') => { last_open = None; },
                    _ => { panic!("Invalid sequence") },
                }
            }
            '7' => {
                match last_open {
                    None => { last_open = Some('7'); },
                    _ => { panic!("Invalid sequence") },
                }
            }
            'J' => {
                match last_open {
                    Some('F') => { intersections += 1; last_open = None; },
                    Some('7') => { last_open = None; },
                    _ => { panic!("Invalid sequence") },
                }
            }
            'F' => {
                match last_open {
                    None => { last_open = Some('F'); },
                    _ => { panic!("Invalid sequence") },
                }
            }
            _ => {}
        }
    }
    intersections % 2 != 0
}

pub fn part_one(input: &str) -> Option<u32> {
    //Some(bfs_count(input.lines().map(|line| line.chars().collect()).collect()))
    let map = bfs(input.lines().map(|line| line.chars().collect()).collect());
    Some(map.into_iter()
        .fold(0, |acc, line| acc + line.iter().filter(|&c| *c != ' ').count() as u32) / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Jordan curve theorem
    let mut map = bfs(input.lines().map(|line| line.chars().collect()).collect());

    // Assign the right char to S
    let start: (usize, usize) = get_start(&map).unwrap();
    let connect: (bool, bool, bool, bool) = (
        start.0 > 0  && ( match map[start.0 - 1][start.1] {
            '|' => true,
            'F' => true,
            '7' => true,
            _ => false,
        }),
        start.1 > 0 && ( match map[start.0][start.1 - 1] {
            '-' => true,
            'F' => true,
            'L' => true,
            _ => false,
        }),
        start.0 < map.len() - 1 && ( match map[start.0 + 1][start.1] {
            '|' => true,
            'J' => true,
            'L' => true,
            _ => false,
        }),
        start.1 < map[0].len() - 1 && ( match map[start.0][start.1 + 1] {
            '-' => true,
            'J' => true,
            '7' => true,
            _ => false,
        }),
    );
    match connect {
        (true, true, false, false) => { map[start.0][start.1] = 'J' },
        (true, false, true, false) => { map[start.0][start.1] = '|' },
        (true, false, false, true) => { map[start.0][start.1] = 'L' },
        (false, true, true, false) => { map[start.0][start.1] = '7' },
        (false, true, false, true) => { map[start.0][start.1] = '-' },
        (false, false, true, true) => { map[start.0][start.1] = 'F' },
        _ => { panic!("Invalid starting point") }
    }

    let mut interior_points: u32 = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if is_interior(i, j, &map) {
                //print!("I");
                interior_points += 1;
            } else {
                //print!("{}", map[i][j]);
            }
        }
        //println!();
    }
    Some(interior_points)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
