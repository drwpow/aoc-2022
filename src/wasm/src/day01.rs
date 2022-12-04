use crate::Output;

pub fn run(input: &str) -> Output {
    // part 1

    // parse & collect data
    let elves_list = input.trim().split("\n\n");
    let mut calories_list: Vec<Vec<i32>> = Vec::new();
    for elf in elves_list {
        let item_list = elf.split("\n");
        let mut calorie_list: Vec<i32> = Vec::new();
        for item in item_list {
            let calorie_count: i32 = item.parse().unwrap();
            calorie_list.push(calorie_count);
        }
        calories_list.push(calorie_list);
    }

    // collect sums
    let mut calorie_sums: Vec<i32> = Vec::new();
    for elf in calories_list {
        calorie_sums.push(elf.iter().sum())
    }

    // sort
    calorie_sums.sort_by(|a, b| b.cmp(a));

    // part 1
    let part1 = calorie_sums[0].to_string().to_owned();

    // part 2
    let part2 = (calorie_sums[0] + calorie_sums[1] + calorie_sums[2])
        .to_string()
        .to_owned();

    return Output {
        part1: part1,
        part2: part2,
    };
}
