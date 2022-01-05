#[cfg(test)]
use std::fs;
use std::{collections::HashSet, hash::Hash};

use crate::submarine::bingo::{build_boards, extraction, Board};

fn vec_equals<T>(a: &[T], b: &[T]) -> bool
where
    T: Eq + Hash,
{
    let a: HashSet<_> = a.iter().collect();
    let b: HashSet<_> = b.iter().collect();

    a == b
}

#[test]
fn extraction_number() {
    let contents =
        fs::read_to_string("data/04-test").expect("Something went wrong reading the file");

    let test_case: Vec<u8> = vec![
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];
    let extraction = extraction(&contents);

    assert!(vec_equals(&test_case, &extraction))
}

#[test]
fn read_board() {
    let contents =
        fs::read_to_string("data/04-test").expect("Something went wrong reading the file");

    let mut columns: Vec<Vec<u8>> = Vec::new();
    columns.push(vec![22, 8, 21, 6, 1]);
    columns.push(vec![13, 2, 9, 10, 12]);
    columns.push(vec![17, 23, 14, 3, 20]);
    columns.push(vec![11, 4, 16, 18, 15]);
    columns.push(vec![0, 24, 7, 5, 19]);

    let mut rows: Vec<Vec<u8>> = Vec::new();
    rows.push(vec![22, 13, 17, 11, 0]);
    rows.push(vec![8, 2, 23, 4, 24]);
    rows.push(vec![21, 9, 14, 16, 7]);
    rows.push(vec![6, 10, 3, 18, 5]);
    rows.push(vec![1, 12, 20, 15, 19]);

    let boards = build_boards(&contents);
    for i in &boards {
        println!("{:?}", i)
    }

    assert!(vec_equals(&boards[0].rows[0], &rows[0]));
    assert!(vec_equals(&boards[0].columns[0], &columns[0]));
}
