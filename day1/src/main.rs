use std::{collections::HashMap};

fn main() {
    let mut gnomes = HashMap::new();
    //let input = include_str!("example_input.txt");
    let input = include_str!("input.txt");


    let mut gnome_id: u32 = 0;
    let mut gnome_calories: u32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            gnomes.insert(format!("gnome{}", gnome_id), gnome_calories);
            gnome_id += 1;
            gnome_calories = 0;
        } else {
            gnome_calories += line.parse::<u32>().unwrap();
        }
        
    }
    let gnome_with_most_calories = gnomes.iter().max_by_key(|entry | entry.1).unwrap();
    println!("{}: {}", gnome_with_most_calories.0, gnome_with_most_calories.1);
}
