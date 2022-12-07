use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut calories_scoreboard = Vec::new();
    //let input = include_str!("example_input.txt");
    let input = include_str!("input.txt");

    let mut calories: u32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            calories_scoreboard.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<u32>().unwrap();
        }
        
    }
    calories_scoreboard.sort();

    if args.len() > 1 {
        // part2 switch by just having input args
        let top_three_sum: u32 = calories_scoreboard.iter().rev().take(3).sum();
        println!("{}", top_three_sum);
    } else {
        println!("{:?}", calories_scoreboard[calories_scoreboard.len() - 1])
    }
    
}
