use log::trace;

pub fn single_sweep_increase(input: String) -> i32 {
    let mut increase: i32 = -1;
    let mut prev = 0;
    let lines = input.lines();
    for elem in lines {
        let curr = elem.parse::<i32>().unwrap();
        if curr > prev {
            increase += 1;
            trace!("increased");
        }
        prev = curr;
    }

    increase
}

pub fn triple_sweep_increse(input: String) -> i32 {
    let mut increase: i32 = -1;
    let mut prev = 0;
    let lines = input.lines();

    for i in 2..input.lines().count() {
        let triplet: Vec<i32> = lines
            .clone()
            .enumerate()
            .filter(|&(j, _)| j >= i - 2 && j <= i)
            .map(|(_, val)| val.parse().expect("parsing error"))
            .collect();

        trace!("{:?}", triplet);
        let curr = triplet[0] + triplet[1] + triplet[2];

        if curr > prev {
            increase += 1;
        }
        prev = curr;
    }

    increase
}

#[test]
fn single_sweep() {
    let contents =
        std::fs::read_to_string("data/01-test").expect("Something went wrong reading the file");
    let result = single_sweep_increase(contents);
    assert!(result == 7);
}

#[test]
fn multiple_sweep() {
    let contents =
        std::fs::read_to_string("data/01-test").expect("Something went wrong reading the file");
    let result = triple_sweep_increse(contents);
    assert!(result == 5);
}
