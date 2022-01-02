use std::fs;
mod tests;

struct Board {
    rows: Vec<Vec<u8>>,
    column: Vec<Vec<u8>>,
}

pub fn resolve(data: String) {
    // read extractions
    let numbers = extraction(data.clone());
    
    // read boards
    // calculate winning board
    // calculate win board result
}

pub fn extraction(data: String) -> Vec<u8> {
    let raw = data.lines().nth(0).unwrap();
    let extraction: Vec<u8> = raw
        .split(',')
        .map(|x| {
            x.parse::<u8>()
                .expect("Error parsing bingo extraction numbers")
        })
        .collect();

    extraction
}
