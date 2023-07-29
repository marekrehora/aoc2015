use regex::Regex;
use std::cmp;
use std::fmt;
use std::fs;

fn day1() {
    println!("Day 1");
    let input = fs::read_to_string("input_1.txt").expect("Input file not found");

    let result = input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    });

    println!("The floor is {}", result);
}

fn day2() {
    println!("Day 2");
    let input = fs::read_to_string("input_2.txt").expect("Input file not found");

    struct Box {
        x: u32,
        y: u32,
        z: u32,
    }

    impl fmt::Display for Box {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "(x:{}, y:{}, z:{})", self.x, self.y, self.z)
        }
    }

    fn from_str(box_string: &str) -> Option<Box> {
        // println!("from_str");
        let re = Regex::new(r#"^([0-9]*)x([0-9]*)x([0-9]*)"#).unwrap();
        let (_, [x, y, z]) = re.captures(box_string).unwrap().extract();

        match (x.parse::<u32>(), y.parse::<u32>(), z.parse::<u32>()) {
            (Ok(x), Ok(y), Ok(z)) => Some(Box { x, y, z }),
            _ => None,
        }
    }

    let boxes = input
        .lines()
        .flat_map(|l| from_str(l))
        .collect::<Vec<Box>>();
    println!("They have {} boxes", boxes.len());

    let paper_area: u32 = boxes
        .iter()
        .map(|b| {
            let first = b.x * b.y;
            let second = b.y * b.z;
            let third = b.x * b.z;
            let smallest = cmp::min(cmp::min(first, second), cmp::min(second, third));

            2 * first + 2 * second + 2 * third + smallest
        })
        .sum();

    println!(
        "Santa will need {} square feet of paper to wrap all the presents",
        paper_area
    );
}

fn main() {
    day1();
    day2();
}
