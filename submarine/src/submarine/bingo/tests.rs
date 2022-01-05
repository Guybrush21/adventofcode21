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

    let boards = build_boards(&contents);
    for i in &boards {
        println!("{:?}", i)
    }

    assert_eq!(&boards[1].columns[2][2], &7);
    assert_eq!(&boards[1].rows[4][4], &6);
    assert_eq!(&boards[0].columns[4][0], &0);
    assert_eq!(&boards[2].rows[4][3], &3);
    assert_eq!(&boards[2].columns[3][3], &6);
}

#[test]
fn board_dont_win() {
    let contents =
        fs::read_to_string("data/04-test").expect("Something went wrong reading the file");

    let boards = build_boards(&contents);
    let extraction = extraction(&contents);

    assert!(!boards[0].is_winning(vec![0, 1, 3]));
    assert!(!boards[0].is_winning(vec![22, 70, 44, 55, 0, 1, 3]));
    assert!(!boards[2].is_winning(vec![17, 23, 2, 0, 14, 21]));
}

#[test]
fn board_win_row() {
    let contents =
        fs::read_to_string("data/04-test").expect("Something went wrong reading the file");

    let boards = build_boards(&contents);
    let extraction = extraction(&contents);

    assert!(boards[0].is_winning(vec![22, 13, 17, 11, 0]));
    assert!(boards[0].is_winning(vec![21, 9, 14, 16, 7]));
    assert!(boards[0].is_winning(vec![0, 1, 21, 9, 14, 16, 7]));
    assert!(boards[2].is_winning(vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24]));
}

#[test]
fn board_win_column() {
    let contents =
        fs::read_to_string("data/04-test").expect("Something went wrong reading the file");

    let boards = build_boards(&contents);
    let extraction = extraction(&contents);

    assert!(boards[0].is_winning(vec![13, 2, 9, 10, 12]));
}

#[test]
fn calculate_score() {
    let contents =
        fs::read_to_string("data/04-test").expect("Something went wrong reading the file");

    let boards = build_boards(&contents);
    let ex = vec![7u8, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24];

    assert_eq!(boards[2].calculate_score(&ex), 188);
}
