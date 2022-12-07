use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut part = 1;
    if args.len() > 1 {
        part = 2;
    }
    //let input = include_str!("example_input.txt");
    let input = include_str!("input.txt");

    let alphabet = String::from_utf8(
        (b'a'..=b'z').chain(b'A'..=b'Z').collect()
    ).unwrap();

    let mut result: i32 = 0;


    for line in input.lines() {
        let compartments: (&str, &str) = line.split_at(line.len()/2);
        for item in compartments.0.chars() {
            if compartments.1.contains(item) {
                let index = alphabet.chars().position(|c| c == item).unwrap() as i32;
                result += index + 1;
                break
            }
        }
    }

    println!("{}", result);
    
}
