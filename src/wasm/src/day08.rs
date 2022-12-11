use crate::Output;

pub fn run(input: &str) -> Output {
    let trees = parse_input(input);

    // part 1
    let mut visibility_grid: Vec<Vec<u8>> = Vec::new();
    let mut scenic_grid: Vec<Vec<u64>> = Vec::new();
    for (y, row) in trees.iter().enumerate() {
        let mut vis_row: Vec<u8> = Vec::new();
        let mut scenic_row: Vec<u64> = Vec::new();
        for (x, height) in row.iter().enumerate() {
            // number of edges this tree is visible from
            // 4 = visible from all 4 edges
            // 0 = not visible from any edge
            let mut visibility: u8 = 4;
            // North
            // note: counting must always start FROM origin, hence .rev()
            let mut n_score: u64 = 0;
            for y2 in (0..y).rev() {
                n_score += 1;
                if trees[y2][x] >= *height {
                    visibility -= 1;
                    break;
                }
            }
            // South
            let mut s_score: u64 = 0;
            for y2 in (y + 1)..trees.len() {
                s_score += 1;
                if trees[y2][x] >= *height {
                    visibility -= 1;
                    break;
                }
            }
            // East
            let mut e_score: u64 = 0;
            for x2 in (0..x).rev() {
                e_score += 1;
                if trees[y][x2] >= *height {
                    visibility -= 1;
                    break;
                }
            }
            // West
            let mut w_score: u64 = 0;
            for x2 in (x + 1)..trees[0].len() {
                w_score += 1;
                if trees[y][x2] >= *height {
                    visibility -= 1;
                    break;
                }
            }
            vis_row.push(visibility);
            scenic_row.push(n_score * s_score * e_score * w_score);
        }
        visibility_grid.push(vis_row);
        scenic_grid.push(scenic_row);
    }

    let mut visible_count = 0;
    for row in &visibility_grid {
        for visibility in row {
            if visibility > &0 {
                visible_count += 1;
            }
        }
    }

    // part 2
    let mut highest_scenic_score: u64 = 0;
    for row in &scenic_grid {
        for scenic_score in row {
            if scenic_score > &highest_scenic_score {
                highest_scenic_score = *scenic_score;
            }
        }
    }

    return Output {
        part1: visible_count.to_string(),
        part2: highest_scenic_score.to_string(),
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
