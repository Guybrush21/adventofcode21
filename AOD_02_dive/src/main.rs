use std::env;
use std::fs;
use std::string::String;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines = contents.lines();
    let mut distance = 0;
    let mut depth = 0;

    for elem in lines {
        let command: Vec<&str> = elem.split_whitespace().collect();
        let value = command[1].parse::<u32>().unwrap();
        let direction = command[0];

        if direction.starts_with('f') {
            distance += value;
        }
        if direction.starts_with('u') {
            depth -= value;
        }
        if direction.starts_with('d') {
            depth += value;
        }
    }

    println!("Total depth = {}", depth);
    println!("Total distance = {}", distance);
    println!("Distance x Solution = {}", distance * depth);
}
