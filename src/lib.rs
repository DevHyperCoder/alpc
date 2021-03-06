mod constants;

use std::io::stdin;

pub fn run() {
    println!("ALPC");
    println!("{:?}", constants::get_terms());
    println!("{:?}", constants::get_numbers());

    let mut input_str = String::new();

    stdin().read_line(&mut input_str).unwrap();

    println!("Converting: {}",input_str);
}
