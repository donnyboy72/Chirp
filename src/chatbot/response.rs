use crate::chatbot::entry::Entry;

use super::commands::Command;
use super::parser::Intent;

pub fn respond(entry: Entry) -> String {
    match entry {
        Entry::Intent(Intent::Greeting) => String::from("Hello! How can I help you?"),
        Entry::Intent(Intent::Goodbye) => String::from("Goodbye! Have a great day."),
        Entry::Intent(Intent::Question) => String::from("Answer..."),
        Entry::Command(Command::Clear) => todo!(),
        Entry::Command(Command::Help) => todo!(),
        Entry::Command(Command::History) => todo!(),
        Entry::Command(Command::Load) => todo!(),
        Entry::Command(Command::Save) => todo!(),
        Entry::Command(Command::Unknown) => todo!(),
    }
}
