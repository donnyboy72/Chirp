#[derive(Debug)]
pub enum Intent {
    Greeting,
    Goodbye,
    Help,
    Unknown,
}

pub fn parse(input: &str) -> Intent {
    match input.to_lowercase().as_str() {
        "hello" | "hi" | "hey" => Intent::Greeting,

        "bye" | "exit" => Intent::Goodbye,

        "help" => Intent::Help,

        _ => Intent::Unknown,
    }
}
