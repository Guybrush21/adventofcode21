use log::debug;
use std::env;
use std::fs;
use std::string::String;

pub struct Position {
    pub distance: u32,
    pub depth: u32,
}

impl Position {
    pub fn result(&self) -> u32 {
        self.distance * self.depth
    }
}

pub fn complex_aiming(contents: String) -> Position {
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

    debug!("Total depth = {}", depth);
    debug!("Total distance = {}", distance);
    debug!("Final Aim = {}", aim);
    debug!("Distance x Solution = {}", distance * depth);
    Position { depth, distance }
}

pub fn simple_aiming(contents: String) -> Position {
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

    debug!("Total depth = {}", depth);
    debug!("Total distance = {}", distance);
    debug!("Distance x Solution = {}", distance * depth);

    Position { depth, distance }
}

#[test]
fn simple_test() {
    let contents =
        std::fs::read_to_string("data/02-test").expect("Something went wrong reading the file");
    let result = simple_aiming(contents);
    assert!(result.depth == 10);
    assert!(result.distance == 15);
    assert!(result.result() == 150);
}
#[test]
fn complex_test() {
    let contents =
        std::fs::read_to_string("data/02-test").expect("Something went wrong reading the file");
    let result = complex_aiming(contents);
    assert!(result.depth == 60);
    assert!(result.distance == 15);
    assert!(result.result() == 900);
}
