use crate::Output;

pub fn run(input: &str) -> Output {
    let trees = parse_input(input);

    // part 1
    let mut visible: Vec<[usize; 2]> = Vec::new();
    let mut invisible: Vec<[usize; 2]> = Vec::new();
    for (y, row) in trees.iter().enumerate() {
        'scan: for (x, height) in row.iter().enumerate() {
            // edge trees are all visible
            if x == 0 || y == 0 || x == row.len() - 1 || y == trees.len() - 1 {
                visible.push([x, y]);
                continue;
            }
            // x-axis
            for x2 in 0..trees[0].len() {
                if x2 == x {
                    continue; // skip same tree
                }
                if trees[y][x2] < *height {
                    visible.push([x, y]);
                    break 'scan;
                }
            }
            // y-axis
            for y2 in 0..row.len() {
                if y2 == y {
                    continue; // skip same tree
                }
                if trees[y2][x] < *height {
                    visible.push([x, y]);
                    break 'scan;
                }
            }
            invisible.push([x, y]);
        }
    }

    return Output {
        part1: visible.len().to_string(),
        part2: "".to_owned(),
    };
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut rows: Vec<Vec<u32>> = Vec::new();
    for ln in input.lines() {
        let mut columns: Vec<u32> = Vec::new();
        for c in ln.chars() {
            columns.push(c.to_digit(10).unwrap());
        }
        rows.push(columns);
    }
    return rows;
}
