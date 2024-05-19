# cliclr

cliclr is a very simple wrapper for the termcolor crate. cliclr is still in the early stages and will be updated frequently to become a feature-rich crate.

## Add the correct use
You should add something like this at the top of your file:

```rust
use cliclr::{console_line::termcolor::Color, console_line::termcolor::ColorChoice, console_line::termcolor::StandardStream ,ConsoleLine};
```

This currently imports all what you need.

## Create a ConsoleLine object

```rust
let console_line = ConsoleLine{
    text: String::from("Hello world!"),
    color: Color::Red
};
```

## Featured functions

Here's a list all the featured functions the crate has.

### print_colored_text

Prints colored text to the standard output.

```rust
console_line.print_colored_text(&mut stdout);
```

### print_bold_text

Prints colored text that is bold to the standard output.

```rust
console_line.print_bold_text(&mut stdout);
```

### print_background_color_text

Prints colored text that has a specefied background color to the standard output.

```rust
console_line.print_background_color_text(&mut stdout, Color::Green);
```

# Thank you

Thank you for taking a look at this crate. Any feedback is welcome!
