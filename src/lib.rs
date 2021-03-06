mod constants;

use std::io::stdin;
use constants::get_terms;

pub fn run() {
    println!("ALPC");
    println!("{:?}", constants::get_terms());
    println!("{:?}", constants::get_numbers());

    let mut input_str = String::new();

    stdin().read_line(&mut input_str).unwrap();

    println!("Converting: {}",input_str);

    let result = parse(input_str);

    println!("{:?}",result);
}

fn parse(input:String) -> Vec<String>{
    let input = input.to_ascii_uppercase();
    let mut result = Vec::new();

    // Loop through chars ; Append to result vector
    for c in input.chars() {
        // Space to newline
        if c == ' ' {
            result.push(format!("{}","\n"));
            continue;
        }

        if !c.is_alphabetic(){
            result.push(format!("{} ",c));
            continue;
        }

        let index = c as usize;

        if index < 65 {continue; }

        // ASCI reper to array index
        let index = index - 65;

        result.push(format!("{} ",get_terms()[index]));
    }
    result
}
