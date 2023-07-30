use std::collections::HashMap;
use std::fs;

pub fn run() {
    println!("Day 3");
    let input = fs::read_to_string("inputs/input_3.txt").expect("Input file not found");

    #[derive(Eq, PartialEq, Hash, Clone, Copy)]
    struct Coord {
        x: i32,
        y: i32,
    }

    let mut map = HashMap::new();
    map.insert(Coord { x: 0, y: 0 }, 1);

    println!("number of insturcitons: {}", input.chars().count());

    input
        .chars()
        .fold(Coord { x: 0, y: 0 }, |coord, c| match c {
            '^' => {
                let new_coord = Coord {
                    y: coord.y + 1,
                    ..coord
                };
                map.entry(new_coord).and_modify(|e| *e += 1).or_insert(1);
                new_coord
            }
            '>' => {
                let new_coord = Coord {
                    x: coord.x + 1,
                    ..coord
                };
                map.entry(new_coord).and_modify(|e| *e += 1).or_insert(1);
                new_coord
            }
            '<' => {
                let new_coord = Coord {
                    x: coord.x - 1,
                    ..coord
                };
                map.entry(new_coord).and_modify(|e| *e += 1).or_insert(1);
                new_coord
            }
            'v' => {
                let new_coord = Coord {
                    y: coord.y - 1,
                    ..coord
                };
                map.entry(new_coord).and_modify(|e| *e += 1).or_insert(1);
                new_coord
            }
            _ => coord, // Cannot be reached due to filter
        });

    println!("{}", map.get(&Coord { x: 0, y: 0 }).unwrap());

    map.iter().for_each(|(k, v)| {
        println!("x:{} y{}: -> {}", k.x, k.y, v);
    });

    println!("Santa went to {} houses", map.len());
}
