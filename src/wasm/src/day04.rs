use crate::Output;

pub fn run(input: &str) -> Output {
    let mut overlap_count: i32 = 0;
    let mut partial_overlap_count: i32 = 0;

    for assignment in input.trim().lines() {
        let ranges = parse_assignment(&assignment);
        let low_a = ranges[0];
        let high_a = ranges[1];
        let low_b = ranges[2];
        let high_b = ranges[3];

        // complete overlap count
        // A surrounds B, or B surrounds A
        if (low_a <= low_b && high_a >= high_b) || (low_b <= low_a && high_b >= high_a) {
            overlap_count += 1;
            partial_overlap_count += 1;
            continue;
        }

        // partial overlap count
        // this is confusing to read, but basically checks if the
        // lower-starting range encompasses the start of the higher
        if (low_a <= low_b && high_a >= low_b) || (low_b <= low_a && high_b >= low_a) {
            partial_overlap_count += 1;
        }
    }

    return Output {
        part1: overlap_count.to_string().to_owned(),
        part2: partial_overlap_count.to_string().to_owned(),
    };
}

// low A, high A, low B, high B
fn parse_assignment(assignment: &str) -> [i32; 4] {
    let mut ranges: [i32; 4] = [0, 0, 0, 0];
    let mut n: usize = 0;
    for range in assignment.split(",") {
        for bound in range.split("-") {
            ranges[n] = bound.parse::<i32>().unwrap();
            n += 1;
        }
    }
    return ranges;
}
