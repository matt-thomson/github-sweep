use std::{env, process};

fn main() {
    let token = match env::var("GITHUB_SWEEP_TOKEN") {
        Ok(val) => val,
        Err(e) => {
            println!("Error reading GITHUB_SWEEP_TOKEN environment variable: {}", e);
            process::exit(1)
        }
    };

    println!("{}", token);
}
