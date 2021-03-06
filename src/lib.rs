mod config;
mod constants;

use config::Config;
use constants::*;
use std::io::stdin;

use structopt::StructOpt;

pub fn run() {
    let opts = Config::from_args();

    if opts.verbose {
        println!("ALPC");
        println!("{:?}", constants::get_terms());
        println!("{:?}", constants::get_numbers());
    }

    let mut input_str = String::new();

    stdin().read_line(&mut input_str).unwrap();

    if opts.verbose {
        println!("Converting: {}", input_str);
    }

    if opts.lowercase {
        input_str = input_str.to_ascii_uppercase();
    }

    let result = parse(input_str);

    for i in result {
        print!("{}", i);
    }
}

pub fn parse(input: String) -> Vec<String> {
    let input = input;
    let mut result = Vec::new();

    // Loop through chars ; Append to result vector
    for c in input.chars() {
        if c.is_digit(10) {
            let num = c.to_digit(10).unwrap() as usize;

            result.push(format!("{} ", get_numbers()[num]).to_ascii_uppercase());
            continue;
        }

        if !c.is_alphabetic() {
            result.push(format!("{}", c));
            continue;
        }

        if c.is_lowercase() {
            result.push(format!("{}", c));
            continue;
        }

        let index = c as usize;

        if index < 65 {
            continue;
        }

        // ASCI reper to array index
        let index = index - 65;

        result.push(format!("{} ", get_terms()[index]).to_ascii_uppercase());
    }
    result
}
