use log::info;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::fs;
mod submarine;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_utc_timestamps()
        .init()
        .unwrap();

    sonar_sweep("data/01-input");
    dive("data/02-input");
    diagnostic("data/03-input");
    bingo("data/04-input");
}

/// Day 01
fn sonar_sweep(datafile: &str) {
    use submarine::sonar;
    info!("==== DAY 01 - PART ONE ====");

    let contents = fs::read_to_string(datafile).expect("Something went wrong reading the file");

    let single_increase = sonar::single_sweep_increase(contents.clone());
    let triple_increase = sonar::triple_sweep_increse(contents.clone());

    info!("Total single increase = {}", single_increase);

    info!("==== DAY 01 - PART TWO ====");
    info!("Total triple increase = {}", triple_increase);
}

/// Day 02
fn dive(datafile: &str) {
    use submarine::aiming_solution;

    let contents = fs::read_to_string(datafile).expect("Something went wrong reading the file");

    info!("==== DAY 02 - PART ONE ====");
    let simple = aiming_solution::simple_aiming(contents.clone());
    info!(
        "Depth: {}, Distance: {}, Result: {} ",
        simple.depth,
        simple.distance,
        simple.result()
    );

    info!("==== DAY 02 - PART TWO ====");
    let complex = aiming_solution::complex_aiming(contents.clone());
    info!(
        "Depth: {}, Distance: {}, Result: {} ",
        complex.depth,
        complex.distance,
        complex.result()
    );
}
/// Day 03
fn diagnostic(datafile: &str) {
    use submarine::life_support;

    info!("==== DAY 03 - PART ONE ====");

    let contents = fs::read_to_string(datafile).expect("Something went wrong reading the file");
    let lines = Vec::from_iter(contents.lines());

    let epsilon = life_support::calculate_epsilon(&lines);
    let gamma = life_support::calculate_gamma(&lines);

    info!("Gamma Rate: {}", gamma);
    info!("Epsilon Rate: {}", epsilon);
    info!("Final result: {}", life_support::calculate_power(&lines));

    info!("==== DAY 03 - PART TWO ====");

    let oxygen = life_support::calculate_oxygen(&lines);
    let co2_scrubber = life_support::calculate_co2(&lines);
    info!("Oxygen value: {}", oxygen);
    info!("CO2 value: {}", co2_scrubber);
    info!("Final result: {}", oxygen * co2_scrubber);
}

/// Day 04 bingo
fn bingo(datafile: &str) {
    use submarine::bingo;
    info!("==== DAY 04 - PART ONE ====");
    let data = fs::read_to_string(datafile).expect("Something went wrong reading the file");
    bingo::play(&data);
}
