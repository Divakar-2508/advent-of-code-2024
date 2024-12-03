mod days;

use days::*;

fn main() {
    if let Some(day) = std::env::args().nth(1) {
        match day.trim() {
            "1" => day01::run(),
            "2" => day02::run(),
            "3" => day03::run(),
            _ => println!("Not impl'd yet!"),
        }
    } else {
        println!("Please specify the day!\neg: cargo run -- 1")
    };
}
