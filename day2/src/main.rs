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

fn translate_code(code: &str, part: i32) -> i32 {
    if part == 1 {
        match code {
            "A" => 1, // rock
            "B" => 2, // paper
            "C" => 3, // scissor
            "X" => 1, // rock
            "Y" => 2, // paper
            "Z" => 3, // scissor
            &_ => 0 // fallback
        }
    } else {
        match code {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            &_ => 0
        }
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

fn clamp_wrap(value: i32) -> i32 {
    // Clamps and wraps value between min and max, if greater than max returns
    // min and if lesser than min returns max
    let min = 1;
    let max = 3;
    if value > max {
        return min;
    } else if value < min {
        return max;
    } else {
        return value;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut part = 1;
    if args.len() > 1 {
        part = 2;
    }

    //let input: &str = include_str!("example_input.txt");
    let input: &str = include_str!("input.txt");
    let mut total_score: i32 = 0;

    for round in input.lines() {
        let mut codes = round.split_whitespace();
        if part == 1 {
            let opponent_shape = translate_code(codes.next().unwrap(), part);
            let my_shape = translate_code(codes.next().unwrap(), part);
            let result: i32 = opponent_shape - my_shape;
            total_score += my_shape + get_score(result);
        } else {
            let shape = translate_code(codes.next().unwrap(), part);
            let outcome = translate_code(codes.next().unwrap(), part);
            total_score += match outcome {
                0 => clamp_wrap(shape - 1) + 0,
                3 => clamp_wrap(shape) + 3,
                6 => clamp_wrap(shape + 1) + 6,
                i32::MIN..=-3_i32 | 3_i32..=i32::MAX => todo!(),
                -2_i32..=-1_i32 | 1_i32..=2_i32 => todo!()
            };
        }
    }

    println!("Total score: {}", total_score);  

}
