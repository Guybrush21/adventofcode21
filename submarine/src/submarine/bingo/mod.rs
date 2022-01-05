use std::collections::HashSet;

use log::debug;

mod tests;

#[derive(Debug)]
pub struct Board {
    pub rows: Vec<Vec<u8>>,
    pub columns: Vec<Vec<u8>>,
}

impl Board {
    pub fn new(rows: Vec<Vec<u8>>) -> Board {
        let mut board = Board {
            rows: rows,
            columns: Vec::new(),
        };

        board.create_column();

        board
    }

    fn create_column(&mut self) {
        for j in 0..self.rows.len() {
            let mut col: Vec<u8> = Vec::with_capacity(self.rows.len());
            for i in 0..self.rows.len() {
                col.push(self.rows[i][j]);
            }
            self.columns.push(col);
        }
    }

    pub fn is_winning(&self, extractions: Vec<u8>) -> bool {
        if extractions.len() < 5 {
            return false;
        }

        self.is_winning_row(&extractions) || self.is_winning_column(&extractions)
    }

    fn is_winning_row(&self, extractions: &Vec<u8>) -> bool {
        let mut win = false;
        for i in &self.rows {
            win = win || i.iter().all(|f| extractions.contains(f))
        }
        win
    }

    fn is_winning_column(&self, extractions: &Vec<u8>) -> bool {
        let mut win = false;
        for i in &self.columns {
            win = win || i.iter().all(|f| extractions.contains(f))
        }
        win
    }

    fn calculate_score(&self, extractions: &Vec<u8>) -> u32 {
        let mut numbers: HashSet<u8> = HashSet::with_capacity(25);

        for i in self.rows.concat() {
            numbers.insert(i);
        }
        for i in self.columns.concat() {
            numbers.insert(i);
        }

        let unmarked: Vec<&u8> = numbers
            .iter()
            .filter(|x| !extractions.contains(x))
            .collect();
        let sum: u32 = unmarked.iter().map(|&&b| b as u32).sum();
        sum
    }
}

pub fn resolve(data: String) {
    // read extractions
    let numbers = extraction(&data);
    let board = build_boards(&data);

    debug!("{:?}", numbers);
    debug!("{:?}", board);

    // read boards
    // calculate winning board
    // calculate win board result
}

pub fn extraction(data: &str) -> Vec<u8> {
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

pub fn build_boards(data: &str) -> Vec<Board> {
    let mut raw = data.lines().skip(2);

    let mut boards: Vec<Board> = Vec::new();

    while let Some(x) = raw.next() {
        if x != "" {
            let r0 = x;
            let r1 = raw.next().unwrap();
            let r2 = raw.next().unwrap();
            let r3 = raw.next().unwrap();
            let r4 = raw.next().unwrap();

            let raw_rows = vec![r0, r1, r2, r3, r4];

            let mut rows: Vec<Vec<u8>> = Vec::with_capacity(5);

            for r in raw_rows {
                let row: Vec<u8> = r
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect();

                rows.push(row);
            }

            let b = Board::new(rows);
            boards.push(b);
        }
    }

    boards
}
