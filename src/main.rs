#![allow(unused)]

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

mod modules {
    pub mod time_registration;
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
            modules::time_registration::make_get_request()
                .await
                .expect("Failed to make GET request");
        }
    }
}
