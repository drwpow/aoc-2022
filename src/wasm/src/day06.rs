use std::{char, collections::HashSet};

use crate::Output;

pub fn run(input: &str) -> Output {
    return Output {
        part1: find_start_of_message(input, 4).to_string(),
        part2: find_start_of_message(input, 14).to_string(),
    };
}

fn find_start_of_message(input: &str, len: usize) -> i32 {
    let mut char_buff = "".to_owned();
    let mut i = 0;
    for c in input.chars() {
        char_buff.push(c);
        i += 1;
        // remove first char if buffer > len
        if char_buff.len() > len {
            char_buff.remove(0);
        }
        // if buffer is 4, count chars
        if char_buff.len() == len {
            let mut char_hash: HashSet<char> = HashSet::new();
            for cb in char_buff.chars() {
                char_hash.insert(cb);
            }
            // if hash has all unique chars, stop iterating
            if char_hash.len() == len {
                break;
            }
            // otherwise, continue
        }
    }
    return i;
}
