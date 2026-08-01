use crate::chatbot::{commands::Command, parser::Intent};

use super::{commands, parser};

pub enum Entry {
    Command(commands::Command),
    Intent(parser::Intent),
    //Unknown,
}
