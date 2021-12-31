use std::fs;

pub fn single_sweep_increase(input: String) -> i32 {
    let mut increase: i32 = -1;
    let mut prev = 0;
    let lines = input.lines();
    for elem in lines {
        let curr = elem.parse::<i32>().unwrap();
        if curr > prev {
            increase += 1;
            println!("increased");
        }
        prev = curr;
    }

    increase
}

#[test]
fn single_sweep() {
    let contents = fs::read_to_string("test").expect("Something went wrong reading the file");
    let result = single_sweep_increase(contents);
    assert!(result == 7);
}
