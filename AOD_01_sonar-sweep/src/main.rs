use std::fs;
mod submarine;
fn main() {
    use submarine::sonar;
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");

    let single_increase = sonar::single_sweep_increase(contents);
    println!("Total increas = {}", single_increase);
}
