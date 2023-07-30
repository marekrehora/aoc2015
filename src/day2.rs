use regex::Regex;
use std::cmp;
use std::fmt;
use std::fs;

pub fn run() {
    println!("Day 2");
    let input = fs::read_to_string("inputs/input_2.txt").expect("Input file not found");

    struct Box {
        l: u32,
        w: u32,
        h: u32,
    }

    impl fmt::Display for Box {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "(x:{}, y:{}, z:{})", self.l, self.w, self.h)
        }
    }

    impl Box {
        fn paper_area(&self) -> u32 {
            let first = &self.l * &self.w;
            let second = &self.w * &self.h;
            let third = &self.l * &self.h;
            let smallest = cmp::min(cmp::min(first, second), cmp::min(second, third));
            2 * first + 2 * second + 2 * third + smallest
        }

        fn ribbon_length(&self) -> u32 {
            let mut ordered = [&self.l, &self.w, &self.h];
            ordered.sort();
            (ordered[0] * 2 + ordered[1] * 2) + &self.l * &self.w * &self.h
        }
    }

    fn from_str(box_string: &str) -> Option<Box> {
        let re = Regex::new(r#"^([0-9]*)x([0-9]*)x([0-9]*)"#).unwrap();
        let (_, [x, y, z]) = re.captures(box_string).unwrap().extract();

        match (x.parse::<u32>(), y.parse::<u32>(), z.parse::<u32>()) {
            (Ok(x), Ok(y), Ok(z)) => Some(Box { l: x, w: y, h: z }),
            _ => None,
        }
    }

    let (paper_area, ribbon_length): (u32, u32) = input
        .lines()
        .flat_map(|l| from_str(l))
        .map(|b| (b.paper_area(), b.ribbon_length()))
        .fold((0, 0), |(acc_area, acc_ribbon), (area, ribbon)| {
            (acc_area + area, acc_ribbon + ribbon)
        });

    println!(
        "Santa will need {} square feet of paper and {} feets of ribbon to wrap all the presents",
        paper_area, ribbon_length
    );
}
