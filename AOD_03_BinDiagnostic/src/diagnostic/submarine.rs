use log::{debug};
use crate::diagnostic::report_analysis;

pub fn calculate_gamma(input: &Vec<&str>) -> u32 {
    let gamma_rate = report_analysis::calculate_most_common(&input);
    report_analysis::convert_string_binary_to_int(&gamma_rate)
}
pub fn calculate_epsilon(input: &Vec<&str>) -> u32 {
    let gamma_rate = report_analysis::calculate_most_common(&input);
    let epsilon_rate = report_analysis::invert_bit_string(&gamma_rate);
    report_analysis::convert_string_binary_to_int(&epsilon_rate)
}

pub fn calculate_power(input: &Vec<&str>) -> u32{
    calculate_gamma(input) * calculate_epsilon(input)
}

pub fn calculate_co2(input: &Vec<&str>) -> u32 {
    let mut filtered = input.clone();

    for index in 0..input[0].chars().count() {
        let most_common =
            report_analysis::invert_bit_string(&report_analysis::calculate_most_common(&filtered));
            
            debug!(
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
        debug!("{}",filtered[0]);
        report_analysis::convert_string_binary_to_int(filtered[0])
    }
}

pub fn calculate_oxygen(input: &Vec<&str>) -> u32 {
    let mut filtered = input.clone();

    for index in 0..input[0].chars().count() {
        let most_common = report_analysis::calculate_most_common(&filtered);
        debug!(
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
    report_analysis::convert_string_binary_to_int(filtered[0])
}
