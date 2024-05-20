use termcolor::{Color, ColorSpec, StandardStream, WriteColor};
use crate::console_line::ConsoleLine;

/// Prints colored text to the standard output.
/// 
/// # Parameters
/// - `stdout`: A mutable reference to a `StandardStream` to handle colored output.
/// - `line`: A reference to a `ConsoleLine`.
pub fn print_colored_text(line: &ConsoleLine, stdout: &mut StandardStream){
    stdout.set_color(ColorSpec::new().set_fg(Some(line.color))).unwrap();
    println!("{}", &line.text);
    stdout.reset().unwrap();
}

/// Prints colored text that is bold to the standard output.
/// 
/// # Parameters
/// - `stdout`: A mutable reference to a `StandardStream` to handle colored output.
/// - `line`: A reference to a `ConsoleLine`.
pub fn print_bold_text(line: &ConsoleLine, stdout: &mut StandardStream){
    stdout.set_color(ColorSpec::new().set_fg(Some(line.color)).set_bold(true)).unwrap();
    println!("{}", &line.text);
    stdout.reset().unwrap();
}

/// Prints colored text that has a specefied background color to the standard output.
/// 
/// # Parameters
/// - `line`: A reference to a `ConsoleLine`.
/// - `stdout`: A mutable reference to a `StandardStream` to handle colored output.
/// - `background_color`: A color from termcolor to set the background color of the output.
pub fn print_background_color_text(line: &ConsoleLine, stdout: &mut StandardStream, background_color: Color){
    stdout.set_color(ColorSpec::new().set_fg(Some(line.color)).set_bg(Some(background_color))).unwrap();
    println!("{}", &line.text);
    stdout.reset().unwrap();
}