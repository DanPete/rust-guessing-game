use colored::ColoredString;
use std::fmt;

pub struct MessageBuilder {
    parts: Vec<ColoredString>,
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }

    pub fn add<T: fmt::Display>(mut self, text: T, color: impl Fn(&str) -> ColoredString) -> Self {
        self.parts.push(color(&text.to_string()));
        self
    }

    pub fn print(self) {
        println!(
            "{}",
            self.parts
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
