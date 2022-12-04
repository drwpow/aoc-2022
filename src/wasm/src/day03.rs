use crate::Output;

pub fn run(input: &str) -> Output {
    // part 1
    let mut part1: i32 = 0;
    for rucksack in input.trim().split("\n") {
        let end = rucksack.len();
        let mid = end / 2;
        let matches = compare_compartments(&rucksack[..mid], &rucksack[mid..end]);
        part1 += get_priority(&matches[0]);
    }

    // part 2

    // split into groups
    let mut groups: Vec<[&str; 3]> = Vec::new();
    let mut group_buffer: [&str; 3] = ["", "", ""];
    for (n, rucksack) in input.trim().split("\n").enumerate() {
        group_buffer[n % 3] = rucksack;
        if n % 3 == 2 {
            groups.push(group_buffer.clone());
        }
    }

    let mut part2: i32 = 0;
    for group in groups {
        for c in group[0].chars() {
            if group[1].contains(c) && group[2].contains(c) {
                part2 += get_priority(&c);
                break;
            }
        }
    }

    return Output {
        part1: part1.to_string().to_owned(),
        part2: part2.to_string().to_owned(),
    };
}

fn compare_compartments(a: &str, b: &str) -> Vec<char> {
    let mut used: Vec<bool> = vec![false; b.len()];
    let mut priority_matches: Vec<char> = Vec::new();
    for ac in a.chars() {
        for (n, bc) in b.chars().enumerate() {
            // if this char has already been matched, skip
            if used[n] == true {
                continue;
            }
            // if this is a match, add to priority_matches and mark to skip
            if ac == bc {
                priority_matches.push(ac);
                used[n] = true;
            }
        }
    }
    return priority_matches;
}

// note: typing these all out is stupid
// but I couldn’t find a simpler solution
fn get_priority(c: &char) -> i32 {
    return match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 0,
    };
}
