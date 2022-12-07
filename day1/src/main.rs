fn main() {
    let mut calories_scoreboard = Vec::new();
    let input = include_str!("example_input.txt");
    //let input = include_str!("input.txt");

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
    println!("{:?}", calories_scoreboard[calories_scoreboard.len() - 1])
}
