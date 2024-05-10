pub extern  crate termcolor;

use termcolor::{ColorSpec, StandardStream, WriteColor, Color};

pub struct ConsoleLine{
    pub text: String,
    pub color: termcolor::Color
}

impl ConsoleLine{
    pub fn print_colored_text(&self, stdout: &mut StandardStream){
        stdout.set_color(ColorSpec::new().set_fg(Some(self.color))).unwrap();
        println!("{}", &self.text);
        stdout.reset().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::stdout;

    #[test]
    fn test_print_colored_text() {
        let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);
        let console_line = ConsoleLine {
            text: String::from("Test text"),
            color: termcolor::Color::Red, // Example color
        };
        console_line.print_colored_text(&mut stdout);
        // Add a read operation to prevent the terminal from closing immediately
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
    }
}