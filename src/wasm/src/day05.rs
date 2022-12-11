use crate::InputDay05;
use crate::Output;

pub fn run(input: InputDay05) -> Output {
    let instructions = parse_instructions(&input.instructions);

    // part 1
    let mut stacks = input.stacks.clone();
    for instruction in &instructions {
        for _ in 0..instruction.count {
            let last = stacks[instruction.from].pop().unwrap();
            stacks[instruction.to].push(last);
        }
    }
    let mut part_1 = "".to_owned();
    for stack in stacks {
        part_1.push_str(stack.last().unwrap());
    }

    // part 2
    let mut stacks_2 = input.stacks.clone();
    for instruction in &instructions {
        let at = stacks_2[instruction.from].len() - instruction.count;
        let tail = stacks_2[instruction.from].split_off(at);
        stacks_2[instruction.to].extend(tail);
    }
    let mut part_2 = "".to_owned();
    for stack in stacks_2 {
        part_2.push_str(stack.last().unwrap());
    }

    return Output {
        part1: part_1,
        part2: part_2,
    };
}

struct MoveCrates {
    from: usize,
    to: usize,
    count: usize,
}

fn parse_instructions(instructions: &str) -> Vec<MoveCrates> {
    let mut parsed: Vec<MoveCrates> = Vec::new();
    for ln in instructions.lines() {
        let parts: Vec<&str> = ln.split(" ").collect();
        parsed.push(MoveCrates {
            from: parts[3].parse::<usize>().unwrap() - 1,
            to: parts[5].parse::<usize>().unwrap() - 1,
            count: parts[1].parse::<usize>().unwrap(),
        })
    }
    return parsed;
}
