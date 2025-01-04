pub enum Color {
    Red, Green, Blue, Yellow,
    Cyan, Magenta, White, Black,
    BrightRed, BrightGreen, Reset,
}

/* Constants */
const TITLE_WIDTH: usize = 16;
lazy_static::lazy_static! {
    static ref DEBUG: bool = {
        let a = std::env::var("DEBUG_LOG").unwrap();
        println!("[[[[[[[[[[[[[DEBUG_LOG env: {a}]]]]]]]]]]]]]");
        a.parse::<bool>().unwrap()
    };
}

/// Diffrent methods for printing debug stuff
pub mod log {
    use std::fmt::Display;
    use super::Color;

    /// Logs if in debug mode
    pub fn log(color: Color, title: &str, text: impl Display) -> () {
        let padding = (super::TITLE_WIDTH - 2).checked_sub(title.len()).unwrap_or(0);

        if *super::DEBUG {
            println!("[{}]{} {}", get_colored(color, title), " ".repeat(padding), text)
        }
    }
    pub fn blue(title: &str, text: impl Display) -> () { log(Color::Blue, title, text) }
    pub fn red(title: &str, text: impl Display) -> () { log(Color::Red, title, text) }
    pub fn green(title: &str, text: impl Display) -> () { log(Color::Green, title, text) }
    pub fn yellow(title: &str, text: impl Display) -> () { log(Color::Yellow, title, text) }
    pub fn cyan(title: &str, text: impl Display) -> () { log(Color::Cyan, title, text) }
    pub fn magenta(title: &str, text: impl Display) -> () { log(Color::Magenta, title, text) }
    pub fn white(title: &str, text: impl Display) -> () { log(Color::White, title, text) }
    pub fn black(title: &str, text: impl Display) -> () { log(Color::Black, title, text) }
    pub fn bright_red(title: &str, text: impl Display) -> () { log(Color::BrightRed, title, text) }
    pub fn bright_green(title: &str, text: impl Display) -> () { log(Color::BrightGreen, title, text) }

    fn get_colored(color: Color, text: &str) -> String {
        let color_code = match color {
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Blue => "\x1b[34m",
            Color::Yellow => "\x1b[33m",
            Color::Cyan => "\x1b[36m",
            Color::Magenta => "\x1b[35m",
            Color::White => "\x1b[37m",
            Color::Black => "\x1b[30m",
            Color::BrightRed => "\x1b[91m",
            Color::BrightGreen => "\x1b[92m",
            Color::Reset => "\x1b[0m",
        };

        return format!("{}{}{}", color_code, text, "\x1b[0m");
    }
}
