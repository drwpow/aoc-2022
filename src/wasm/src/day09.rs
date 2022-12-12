use crate::Output;
use std::collections::HashSet;

pub fn run(input: &str) -> Output {
    let mut tail_visited: HashSet<[i32; 2]> = HashSet::from([[0, 0]]);
    let mut last_head: [i32; 2] = [0, 0];
    let mut last_tail: [i32; 2] = [0, 0];
    for ln in input.lines() {
        let parts = ln.split(" ").collect::<Vec<&str>>();
        let direction = parts[0];
        let count = parts[1].parse::<i32>().unwrap();
        for _ in 0..count {
            let mut next_head_x: i32 = last_head[0].clone();
            let mut next_head_y: i32 = last_head[1].clone();
            match direction {
                "U" => {
                    next_head_y += 1;
                }
                "D" => {
                    next_head_y -= 1;
                }
                "L" => {
                    next_head_x -= 1;
                }
                "R" => {
                    next_head_x += 1;
                }
                _ => {}
            };
            let mut next_tail_x = last_tail[0].clone();
            let mut next_tail_y = last_tail[1].clone();
            let delta_x = next_head_x - next_tail_x;
            let delta_y = next_head_y - next_tail_y;
            // handle corners
            if delta_x != 0 && delta_y != 0 && (delta_x.abs() > 1 || delta_y.abs() > 1) {
                // if moving in X axis, align tail’s Y axis, and vice-versa
                if delta_x.abs() > 1 {
                    if delta_x > 0 {
                        next_tail_x += 1;
                    } else {
                        next_tail_x -= 1;
                    }
                    next_tail_y = next_head_y;
                } else {
                    if delta_y > 0 {
                        next_tail_y += 1;
                    } else {
                        next_tail_y -= 1;
                    }
                    next_tail_x = next_head_x;
                }
            }
            // straight moves
            else if delta_x.abs() > 1 {
                if delta_x > 0 {
                    next_tail_x += 1;
                } else {
                    next_tail_x -= 1;
                }
            } else if delta_y.abs() > 1 {
                if delta_y > 0 {
                    next_tail_y += 1;
                } else {
                    next_tail_y -= 1;
                }
            }
            last_head = [next_head_x, next_head_y];
            last_tail = [next_tail_x, next_tail_y];
            tail_visited.insert([next_tail_x, next_tail_y]);
        }
    }

    return Output {
        part1: tail_visited.len().to_string(),
        part2: "".to_owned(),
    };
}
