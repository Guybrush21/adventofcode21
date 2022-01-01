use log::{trace, debug};

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
            trace!("{:?}", counter);
        }
    }
    debug!("coutner: {:?}", counter );
   
    for bit in counter {
        debug!("bit: {} - len: {}", bit, input.len() as u32 / 2 );
        let c =         
        if bit > (input.len() as u32/ 2) 
        || (input.len() % 2 == 0 && bit == (input.len() as u32/ 2)) // manage odd dimension
        {
            "1"
        } else {
            "0"
        };
        rate.push_str(c);
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
