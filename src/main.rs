use pest::Parser;
use anyhow::anyhow;
use std::env;
use std::fs;
use sql_parser::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "--help" => print_help(),
        "--credits" => print_credits(),
        input_file => {
            let file_content = fs::read_to_string(input_file).expect("Could not open file.");
            match parse_sql(&file_content) {
                Ok(parsed_data) => println!("{}", parsed_data),
                Err(e) => eprintln!("Parsing error: {}", e),
            }
        }
    }
}


fn print_help() {
    println!("SQL query parser:");
    println!("  cargo run <input_file>     Parses an SQL query file and displays its components.");
    println!("  cargo run -- --help        Displays help information.");
    println!("  cargo run -- --credits     Shows project credits.");
}

fn print_credits() {
    println!("Author: Daria Vetrykush");
}