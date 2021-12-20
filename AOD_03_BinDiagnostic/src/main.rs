use std::fs;

fn main() {
    println!("Solving part 1 of third AOD");

    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());
    let total = lines.len() as u32;
    let input_size = lines[0].chars().count();

    let mut gamma_rate = String::with_capacity(input_size);
    let mut counter: Vec<u32> = Vec::with_capacity(input_size);

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

    for bit in counter {
        gamma_rate.push_str(if bit > (total / 2) { "1" } else { "0" });
    }
    println!("Gamma Rate: {:?}", gamma_rate);
    let epsilon_rate = invert_bit_string(&gamma_rate);
    println!("Gamma Rate: {:?}", epsilon_rate);

    let epsilon = convert_string_binary_to_int(&epsilon_rate);
    let gamma = convert_string_binary_to_int(&gamma_rate);
    let result = epsilon * gamma;

    println!("Gamma Rate: {}", gamma);
    println!("Epsilon Rate: {}", epsilon);
    println!("Final result: {}", result);

    // TODO: convert binary string to real number and multiply those
}

fn convert_string_binary_to_int(val: &str) -> u32 {
    let intval = u32::from_str_radix(val, 2).unwrap();
    return intval;
}

fn invert_bit_string(array: &str) -> std::string::String {
    let mut result = String::with_capacity(array.len());
    for i in array.chars() {
        result.push_str(if i == '1' { "0" } else { "1" });
    }
    result
}
