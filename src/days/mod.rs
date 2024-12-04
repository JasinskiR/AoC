pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

pub fn run_day(day: &str, mode: bool) -> Result<(), String> {
    match day {
        "1" => day01::run(mode),
        "2" => day02::run(mode),
        "3" => day03::run(mode),
        "4" => day04::run(mode),
        // Add more days here...
        _ => return Err(format!("Day {} is not implemented!", day)),
    }
    Ok(())
}
