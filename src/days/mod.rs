pub mod day01;

pub fn run_day(day: &str, mode: bool) -> Result<(), String> {
    match day {
        "1" => day01::run(mode),
        // Add more days here...
        _ => return Err(format!("Day {} is not implemented!", day)),
    }
    Ok(())
}