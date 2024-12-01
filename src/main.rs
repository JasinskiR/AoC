mod days; 

use std::env;

#[repr(i32)]
#[derive(PartialEq)]
enum Mode {
    Test = 1,
    Day = 0,
}

impl Mode {
    // Convert string to Mode enum (case-insensitive)
    fn from_str(input: &str) -> Result<Self, &'static str> {
        match input.to_lowercase().as_str() {
            "test" => Ok(Mode::Test),
            "day" => Ok(Mode::Day),
            _ => Err("Invalid mode. Use 'test' or 'day'."),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: cargo run <day> <mode>");
        return;
    }

    let day = &args[1];
    

    let mode = if args.len() > 2 {
        Mode::from_str(&args[2]).unwrap_or(Mode::Test)
    } else {
        Mode::Test
    };

    match days::run_day(day, mode == Mode::Test) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
