use termcolor::{Color, ColorSpec, StandardStream, WriteColor};
use crate::console_line::ConsoleLine;
use std::io::Write;

/// Prints colored text to the standard output.
/// 
/// # Parameters
/// - `stdout`: A mutable reference to a `StandardStream` to handle colored output.
/// - `line`: A reference to a `ConsoleLine`.
pub fn print_colored_text(line: &ConsoleLine, stdout: &mut StandardStream){
    stdout.set_color(ColorSpec::new().set_fg(Some(line.color))).unwrap();
    writeln!(stdout, "{}", &line.text).unwrap();
    stdout.reset().unwrap();
}

/// Prints colored text that is bold to the standard output.
/// 
/// # Parameters
/// - `stdout`: A mutable reference to a `StandardStream` to handle colored output.
/// - `line`: A reference to a `ConsoleLine`.
pub fn print_bold_text(line: &ConsoleLine, stdout: &mut StandardStream){
    stdout.set_color(ColorSpec::new().set_fg(Some(line.color)).set_bold(true)).unwrap();
    writeln!(stdout, "{}", &line.text).unwrap();
    stdout.reset().unwrap();
}

/// Prints colored text that has a specified background color to the standard output.
/// 
/// # Parameters
/// - `line`: A reference to a `ConsoleLine`.
/// - `stdout`: A mutable reference to a `StandardStream` to handle colored output.
/// - `background_color`: A color from termcolor to set the background color of the output.
pub fn print_background_color_text(line: &ConsoleLine, stdout: &mut StandardStream, background_color: Color){
    stdout.set_color(ColorSpec::new().set_fg(Some(line.color)).set_bg(Some(background_color))).unwrap();
    writeln!(stdout, "{}", &line.text).unwrap();
    stdout.reset().unwrap();
}

#[cfg(test)]
mod tests {
    use termcolor::ColorChoice;
    use super::*;
    #[test]
    pub fn test_print_background_color_text(){
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let console_line = ConsoleLine {
            text: String::from("A first test."),
            color: Color::Cyan,
        };
        print_background_color_text(&console_line, &mut stdout, Color::Magenta);
    }

    #[test]
    pub fn test_print_bold_text(){
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let console_line = ConsoleLine {
            text: String::from("A second test."),
            color: Color::Cyan,
        };
        print_bold_text(&console_line, &mut stdout);
    }

    #[test]
    pub fn test_print_colored_text(){
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let console_line = ConsoleLine {
            text: String::from("A third test."),
            color: Color::Yellow,
        };
        print_colored_text(&console_line, &mut stdout);
    }
}