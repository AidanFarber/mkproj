use json;
use std::env;
use std::process::Command;

// ---- Function Definitions ----
// Get which language environment to setup from command line
fn parse_language(args: Vec<String>) -> String {
    for arg in args {
        match arg.as_str() {
            "python" => print!("stink"),
            _ => print!("you suck"),
        }
    };

    return "".to_string();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    parse_language(args);
    println!("Hello, world!");
}
