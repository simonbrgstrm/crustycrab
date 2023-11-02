use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod checkin;
mod checkout;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// STAGGERING
    #[arg(short, long, action = clap::ArgAction::Count)]
    stagger: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    /// Checkin current time
    Checkin {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    Checkout {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    match cli.stagger {
        0 => println!("No Teams notification"),
        1 => println!("Sending Teams notification"),
        2 => println!("Stagger 2"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        #[allow(unused_variables)]
        Some(Commands::Checkin { list }) => {
            checkin::checkin_time();
        }
        #[allow(unused_variables)]
        Some(Commands::Checkout { list }) => {
            checkout::checkout_time();
        }
        None => {}
    }

    // Continued program logic goes here...
}
