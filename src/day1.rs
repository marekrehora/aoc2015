use std::fs;

pub fn run() {
    println!("Day 1");
    let input = fs::read_to_string("inputs/input_1.txt").expect("Input file not found");

    let result = input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    });

    println!("The floor is {}", result);
}
