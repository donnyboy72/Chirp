#[derive(Debug)]
pub enum Command {
    Save,
    Load,
    Help,
    History,
    Clear,
    Unknown,
}

pub fn parse(input: &str) -> Command {
    match input.to_lowercase().as_str() {
        "save" => Command::Save,
        "load" => Command::Load,
        "help" => Command::Help,
        "history" | "log" => Command::History,
        "clear" | "delete" => Command::Clear,
        _ => Command::Unknown,
    }
}
