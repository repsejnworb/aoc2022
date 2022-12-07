use std::env;

fn part1(data: &str) -> i32 {
    let mut result: i32 = 0;

    return result;
}

fn part2(data: &str) -> i32 {
    let mut result: i32 = 0;
    
    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut part: i32 = 1;
    if args.len() > 1 {
        part = 2;
    }

    if part == 1 {
        let part1_example_result: i32 = part1(include_str!("example_input.txt"));
        assert_eq!(part1_example_result, 2);

        let part1_result: i32 = part1(include_str!("input.txt"));
        assert_eq!(part1_result, 1337);

        println!("Part1 Example: {}", part1_example_result);
        println!("Part1: {}", part1_result);
    } else {
        let part2_example_result: i32 = part2(include_str!("example_input.txt"));
        assert_eq!(part2_example_result, 1337);

        let part2_result: i32 = part2(include_str!("input.txt"));
        assert_eq!(part2_result, 1337);

        println!("Part2 Example: {}", part2_example_result);
        println!("Part2: {}", part2_result);
    }
}
