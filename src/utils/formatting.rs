use std::fmt;

// A trait for colored output
pub trait ColoredOutput {
    fn print_colored(&self, color: impl Fn(&str) -> String);
    fn to_colored(&self, color: impl Fn(&str) -> String) -> String;
}

// Implement for any type that can be converted to a string
impl<T: fmt::Display> ColoredOutput for T {
    fn print_colored(&self, color: impl Fn(&str) -> String) {
        println!("{}", color(&self.to_string()));
    }

    fn to_colored(&self, color: impl Fn(&str) -> String) -> String {
        color(&self.to_string())
    }
}
