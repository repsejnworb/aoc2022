/*
## Opponent
A: Rock
B: Paper
C: Scissor

## Response
X: Rock
Y: Paper
Z: Scissor

## Score

** Per shape _I_ selected
1 for Rock
2 for Paper
3 for Scissors

** Per outcome
0 for loss
3 for draw
6 for win


EXAMPLE:
A Y
B X
C Z


0 == draw

1 = loss
2 = win

-1 = win
-2 = loss

let input = include_str!("input.txt");
let args: Vec<String> = env::args().collect();
*/

use std::env;

fn translate_shape(letter: &str) -> i32 {
    match letter {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        &_ => 0
    }
}

fn get_score(outcome: i32) -> i32 {
    match outcome {
        0 => 3,
        1 => 0,
        2 => 6,
        -1 => 6,
        -2 => 0,
        i32::MIN..=-3_i32 | 3_i32..=i32::MAX => todo!()
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut _part = 1;
    if args.len() > 1 {
        _part = 2;
    }

    //let input: &str = include_str!("example_input.txt");
    let input: &str = include_str!("input.txt");
    let mut total_score: i32 = 0;

    for round in input.lines() {
        let mut round_shapes = round.split_whitespace();
        let round = (round_shapes.next().unwrap(), round_shapes.next().unwrap());
        let result: i32 = translate_shape(round.0) - translate_shape(round.1);
        total_score += translate_shape(round.1);
        total_score += get_score(result);
    }

    println!("Total score: {}", total_score);  

}
