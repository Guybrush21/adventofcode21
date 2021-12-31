use log::info;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::fs;
mod diagnostic;
mod test;
fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_utc_timestamps()
        .init()
        .unwrap();

    info!("==== PART TWO ====");

    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());

    let epsilon = diagnostic::submarine::calculate_epsilon(&lines);
    let gamma = diagnostic::submarine::calculate_gamma(&lines);

    info!("Gamma Rate: {}", gamma);
    info!("Epsilon Rate: {}", epsilon);
    info!(
        "Final result: {}",
        diagnostic::submarine::calculate_power(&lines)
    );

    info!("==== PART TWO ====");

    let oxygen = diagnostic::submarine::calculate_oxygen(&lines);
    let co2_scrubber = diagnostic::submarine::calculate_co2(&lines);
    info!("Oxygen value: {}", oxygen);
    info!("CO2 value: {}", co2_scrubber);
    info!("Final result: {}", oxygen * co2_scrubber);
}
