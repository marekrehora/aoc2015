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

    fn houses_visited(input: &String) -> HashMap<Coord, u32> {
        let mut map = HashMap::new(); // THere may be a way to do this without any mutations
        map.insert(Coord { x: 0, y: 0 }, 1); // The first house is always visited

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
                _ => coord, // invalid char, no movement
            });

        map
    }

    // map.iter().for_each(|(k, v)| {
    //     println!("x:{} y{}: -> {}", k.x, k.y, v);
    // });

    println!("Santa went alone to {} houses", houses_visited(&input).len());

    // Part 2
    // This is probably not the most efficient way but whatever
    let odd_orders = &input.char_indices().filter(|(i, _)| i % 2 != 0).map(|(_, c)| c).collect();
    let even_orders = &input.char_indices().filter(|(i, _)| i % 2 == 0).map(|(_, c)| c).collect();

    let res: HashMap<Coord, u32> = houses_visited(&odd_orders).into_iter().chain(houses_visited(&even_orders)).collect();

    println!("Santa and Robo-Santa went to {} houses", res.len());

}
