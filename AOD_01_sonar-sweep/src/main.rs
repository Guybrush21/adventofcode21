use std::fs;
mod submarine;
fn main() {
    use submarine::sonar;
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");

    let single_increase = sonar::single_sweep_increase(contents.clone());
    let triple_increase = sonar::triple_sweep_increse(contents.clone());

    println!("Total single increase = {}", single_increase);
    println!("Total triple increase = {}", triple_increase);
}
