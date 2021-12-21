use std::fs;

fn main() {
    println!("Solving part 1 of third AOD");

    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());

    let gamma_rate = calculate_most_common(&lines);
    let epsilon_rate = invert_bit_string(&gamma_rate);
    println!("Gamma Rate: {:?}", gamma_rate);
    println!("Epsilon Rate: {:?}", epsilon_rate);

    let epsilon = convert_string_binary_to_int(&epsilon_rate);
    let gamma = convert_string_binary_to_int(&gamma_rate);
    let result = epsilon * gamma;

    println!("Gamma Rate: {}", gamma);
    println!("Epsilon Rate: {}", epsilon);
    println!("Final result: {}", result);

    calculate_oxygen(&lines);
}

fn calculate_oxygen(input: &Vec<&str>) -> String {
    let mut index = 0;
    let result: Vec<u8> = Vec::with_capacity(input[0].chars().count());

    return "".to_string();
}

fn calculate_most_common(input: &Vec<&str>) -> std::string::String {
    let length = input[0].chars().count();
    let mut rate = String::with_capacity(length);
    let mut counter: Vec<u32> = Vec::with_capacity(length);
    for i in 0..length {
        counter.push(0);
    }

    for elem in input {
        for (i, c) in elem.chars().enumerate() {
            if c == '1' {
                counter[i] += 1
            }
            println!("{:?}", counter);
        }
    }
    for bit in counter {
        rate.push_str(if bit > input.len() as u32 / 2 {
            "1"
        } else {
            "0"
        });
    }
    rate
}

fn calculate_most_common_in_position(input: &Vec<&str>, index: &u32) -> u8 {
    let total = input.len();
    8
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
