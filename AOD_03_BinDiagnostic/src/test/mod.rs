#[cfg(test)]
use std::fs;

use log::LevelFilter;
use simple_logger::SimpleLogger;

use crate::diagnostic::submarine;


fn setup(){
    SimpleLogger::new()
    .with_level(LevelFilter::Debug).with_utc_timestamps()
    .init()
    .unwrap();   
}
#[test]
fn file_to_lines() {
    let contents = fs::read_to_string("test").expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());

    assert!(lines.len() != 0);
}

/// gamma rate should be 22
#[test]
fn gamma() {
    let contents = fs::read_to_string("test").expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());
    let r = submarine::calculate_gamma(&lines);
    assert_eq!(r, 22);
}

/// epsilon rate should be 9
#[test]
fn epsilon() {
    let contents = fs::read_to_string("test").expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());
    let r = submarine::calculate_epsilon(&lines);
    assert_eq!(r, 9);
}

/// power should be 198
#[test]
fn power() {
    let contents = fs::read_to_string("test").expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());
    let r = submarine::calculate_power(&lines);
    assert_eq!(r, 198);
}

/// oxygen should be 23
#[test]
fn oxygen() {
    setup();
    let contents = fs::read_to_string("test").expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());
    let r = submarine::calculate_oxygen(&lines);
    assert_eq!(r, 23);
}

/// co2 should be 10
#[test]
fn co2() {
    let contents = fs::read_to_string("test").expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());
    let r = submarine::calculate_co2(&lines);
    assert_eq!(r, 10);
}
