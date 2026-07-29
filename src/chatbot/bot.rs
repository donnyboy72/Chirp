use super::parser;
use super::response;

use std::io;
use std::io::Error;
use std::process;

#[derive(Debug)]
pub struct Bot {
    output: String,
}

impl Bot {
    pub fn new() -> Result<Bot, Error> {
        Ok(Bot {
            output: String::new(),
        })
    }

    pub fn run(&mut self) {
        loop {
            let input_string = get_input().unwrap_or_else(|err| {
                eprintln!("Issue reading input {}", err);
                process::exit(1);
            });

            if input_string.to_lowercase() == "exit".to_string() {
                break;
            }

            let parse = parser::parse(&input_string); //get input and passes to parser
            let output = response::respond(parse);
            self.output = output.to_string();
            println!("\nBot: {}\n", self.output);
        }
    }
}

fn get_input() -> Result<String, Error> {
    println!("Enter text: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Issue reading line");

    let input = input.trim();
    Ok(input.to_string())
}
