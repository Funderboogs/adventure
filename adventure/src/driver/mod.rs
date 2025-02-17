use std::error::Error;
mod console;
mod text;

pub use console::ConsoleDriver;
pub use text::TextDriver;

pub trait Driver<'game> {
    fn drive(&mut self) -> Result<(), Box<dyn Error>>;
}
