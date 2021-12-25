#[cfg(test)]
use std::fs;

#[test]
fn file_to_lines() {
    let contents = fs::read_to_string("test/test").expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());
    
    assert!(lines.len() != 0);
    
}
