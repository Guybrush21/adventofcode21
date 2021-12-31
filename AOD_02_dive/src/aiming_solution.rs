use std::env;
use std::fs;
use std::string::String;

pub fn complex_aiming(contents: String) {
    let lines = contents.lines();
    let mut distance = 0;
    let mut depth = 0;
    let mut aim = 0;

    for elem in lines {
        let command: Vec<&str> = elem.split_whitespace().collect();
        let value = command[1].parse::<u32>().unwrap();
        let direction = command[0];

        if direction.starts_with('f') {
            distance += value;
            depth += value * aim;
        }
        if direction.starts_with('u') {
            aim -= value;
        }
        if direction.starts_with('d') {
            aim += value;
        }
    }

    println!("Total depth = {}", depth);
    println!("Total distance = {}", distance);
    println!("Final Aim = {}", aim);
    println!("Distance x Solution = {}", distance * depth);
}

pub fn simple_aiming(contents: String) {
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
