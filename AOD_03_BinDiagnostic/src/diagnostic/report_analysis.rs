use log::{debug};

pub fn calculate_most_common(input: &Vec<&str>) -> std::string::String {
    let length = input[0].chars().count();
    let mut rate = String::with_capacity(length);
    let mut counter: Vec<u32> = Vec::with_capacity(length);
    for _ in 0..length {
        counter.push(0);
    }

    for elem in input {
        for (i, c) in elem.chars().enumerate() {
            if c == '1' {
                counter[i] += 1
            }
            debug!("{:?}", counter);
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

pub fn convert_string_binary_to_int(val: &str) -> u32 {
    let intval = u32::from_str_radix(val, 2).unwrap();
    return intval;
}

pub fn invert_bit_string(array: &str) -> std::string::String {
    let mut result = String::with_capacity(array.len());
    for i in array.chars() {
        result.push_str(if i == '1' { "0" } else { "1" });
    }
    result
}
