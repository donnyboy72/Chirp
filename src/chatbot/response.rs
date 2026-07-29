use super::parser::Intent;

pub fn respond(intent: Intent) -> String {
    match intent {
        Intent::Greeting => String::from("Hello! How can I help you?"),

        Intent::Goodbye => String::from("Goodbye! Have a great day."),

        Intent::Help => String::from("You can say hello, ask questions, or type exit."),

        Intent::Unknown => String::from("I'm not sure how to answer that yet."),
    }
}
