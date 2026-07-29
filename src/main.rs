mod chatbot;

use chatbot::bot::Bot;
use std::process;

fn main() {
    let mut bot = Bot::new().unwrap_or_else(|err| {
        eprintln!("Issue running bot {}", err);
        process::exit(1);
    });

    bot.run();
}
