use std::fs;

mod aiming_solution;

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");

    aiming_solution::simple_aiming(contents.clone());
    aiming_solution::complex_aiming(contents.clone());
}
