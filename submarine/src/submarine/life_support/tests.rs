#[cfg(test)]
use std::fs;
#[test]
fn file_to_lines() {
    let contents =
        fs::read_to_string("data/03-test").expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());

    assert!(lines.len() != 0);
}

/// gamma rate should be 22
#[test]
fn gamma() {
    let contents =
        fs::read_to_string("data/03-test").expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());
    let r = super::calculate_gamma(&lines);
    assert_eq!(r, 22);
}

/// epsilon rate should be 9
#[test]
fn epsilon() {
    let contents =
        fs::read_to_string("data/03-test").expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());
    let r = super::calculate_epsilon(&lines);
    assert_eq!(r, 9);
}

/// power should be 198
#[test]
fn power() {
    let contents =
        fs::read_to_string("data/03-test").expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());
    let r = super::calculate_power(&lines);
    assert_eq!(r, 198);
}

/// oxygen should be 23
#[test]
fn oxygen() {
    let contents =
        fs::read_to_string("data/03-test").expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());
    let r = super::calculate_oxygen(&lines);
    assert_eq!(r, 23);
}

/// co2 should be 10
#[test]
fn co2() {
    let contents =
        fs::read_to_string("data/03-test").expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());
    let r = super::calculate_co2(&lines);
    assert_eq!(r, 10);
}
