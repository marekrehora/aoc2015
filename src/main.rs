use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input file not found");

    let result = input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    });

    println!("The floor is {}", result);
}
