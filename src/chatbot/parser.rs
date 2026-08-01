use super::entry::Entry;
use crate::chatbot::commands;

#[derive(Debug)]
pub enum Intent {
    Greeting,
    Goodbye,
    Question,
}

// pub fn parse(input: &str) -> Intent {
//     match input.to_lowercase().as_str() {
//         "hello" | "hi" | "hey" => Intent::Greeting,
//         "bye" | "exit" => Intent::Goodbye,
//         "what" | "when" | "where" | "how" | "why" => Intent::Question,
//         _ => Intent::Unknown,
//     }
// }

pub fn parse(input: &str) -> Entry {
    match input.to_lowercase().as_str() {
        "hello" | "hi" | "hey" => Entry::Intent(Intent::Greeting),
        "bye" | "exit" => Entry::Intent(Intent::Goodbye),
        "what" | "when" | "where" | "how" | "why" => Entry::Intent(Intent::Question),
        _ => Entry::Command(commands::parse(&input)),
    }
}
