use log::LevelFilter;
use log::{debug, info, trace};
use simple_logger::SimpleLogger;
use std::fs;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
    info!("Solving part 1 of third AOD");

    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());

    let gamma_rate = calculate_most_common(&lines);
    let epsilon_rate = invert_bit_string(&gamma_rate);
    info!("Gamma Rate: {:?}", gamma_rate);
    info!("Epsilon Rate: {:?}", epsilon_rate);

    let epsilon = convert_string_binary_to_int(&epsilon_rate);
    let gamma = convert_string_binary_to_int(&gamma_rate);
    let result = epsilon * gamma;

    info!("Gamma Rate: {}", gamma);
    info!("Epsilon Rate: {}", epsilon);
    info!("Final result: {}", result);

    info!("==== PART TWO ====");

    let oxygen = calculate_oxygen(&lines);
    info!("Oxygen value: {}", oxygen);

    let co2_scrubber = calculate_co2(&lines);
    info!("CO2 value: {}", co2_scrubber);
}

fn calculate_co2(input: &Vec<&str>) -> u32 {
    let mut filtered = input.clone();

    for index in 0..input[0].chars().count() {
        let most_common = invert_bit_string(&calculate_most_common(&filtered));
        trace!(
            "Searching for {:?} in position {} in {:?}",
            most_common.chars().nth(index),
            index,
            filtered
        );
        filtered.retain(|&x| x.chars().nth(index) == most_common.chars().nth(index));
        if filtered.len() == 1 {
            break;
        };
    }

    if filtered.len() > 1 {
        panic!("Input is undefined?");
    } else {
        convert_string_binary_to_int(filtered[0])
    }
}

fn calculate_oxygen(input: &Vec<&str>) -> u32 {
    let mut filtered = input.clone();

    for index in 0..input[0].chars().count() {
        let most_common = calculate_most_common(&filtered);
        trace!(
            "Searching for {:?} in position {} in {:?}",
            most_common.chars().nth(index),
            index,
            filtered
        );
        filtered.retain(|&x| x.chars().nth(index) == most_common.chars().nth(index));
        if filtered.len() == 1 {
            break;
        };
    }
    convert_string_binary_to_int(filtered[0])
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
