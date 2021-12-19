use std::fs;

fn main() {
    println!("Solving part 1 of third AOD");

    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());
    let total = lines.len() as u32;
    let inputSize = lines[0].chars().count();

    let mut counter: Vec<u32> = Vec::with_capacity(inputSize);
    for i in 0..counter.capacity() {
        counter.push(0)
    }
    println!("{:?}", counter);

    for elem in contents.lines() {
        for (i, c) in elem.chars().enumerate() {
            if c == '1' {
                counter[i] += 1
            }
            println!("{:?}", counter);
        }
    }

    let mut gamma_rate = String::with_capacity(inputSize);
    for bit in counter {
        gamma_rate.push_str(if bit > (total / 2) { "1" } else { "0" });
    }
    println!("Gamma Rate: {:?}", gamma_rate);
    let epsilon_rate = invert_bit_string(&gamma_rate);
    println!("Gamma Rate: {:?}", epsilon_rate);

    // TODO: convert binary string to real number and multiply those
}

fn invert_bit_string(array: &str) -> std::string::String {
    let mut result = String::with_capacity(array.len());
    for i in array.chars() {
        result.push_str(if i == '1' { "0" } else { "1" });
    }
    result
}
