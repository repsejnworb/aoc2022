use std::env;

fn part1(data: &str) -> i32 {

    let alphabet: String = String::from_utf8(
        (b'a'..=b'z').chain(b'A'..=b'Z').collect()
    ).unwrap();

    let mut result: i32 = 0;


    data.lines().for_each(|line: &str| {
        let compartments: (&str, &str) = line.split_at(line.len()/2);
        for item in compartments.0.chars() {
            if compartments.1.contains(item) {
                let index: i32 = alphabet.chars().position(|c| c == item).unwrap() as i32;
                result += index + 1;
                break
            }
        }
    });
    return result;
}

fn part2(data: &str) -> i32 {

    let alphabet: String = String::from_utf8(
        (b'a'..=b'z').chain(b'A'..=b'Z').collect()
    ).unwrap();

    let lines: Vec<&str> = data.lines().collect();
    let mut result: i32 = 0;
    

    for group in lines.chunks(3) {
        for item in group[0].chars() {
            if group[1].contains(item) && group[2].contains(item) {
                let index: i32 = alphabet.chars().position(|c| c == item).unwrap() as i32;
                result += index + 1;
                break
            }
        }
    }
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
        assert_eq!(part1_example_result, 157);

        let part1_result: i32 = part1(include_str!("input.txt"));
        assert_eq!(part1_result, 7766);

        println!("Part1 Example: {}", part1_example_result);
        println!("Part1: {}", part1_result);
    } else {
        let part2_example_result: i32 = part2(include_str!("example_input.txt"));
        assert_eq!(part2_example_result, 70);

        let part2_result: i32 = part2(include_str!("input.txt"));
        assert_eq!(part2_result, 2415);

        println!("Part2 Example: {}", part2_example_result);
        println!("Part2: {}", part2_result);
    }
    
}
