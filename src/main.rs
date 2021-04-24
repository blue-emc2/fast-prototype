use iced::{Application, Settings};

mod content;
mod gui;

use gui::GUI;

pub fn main() -> iced::Result {
  GUI::run(Settings {
    antialiasing: true,
    ..Settings::default()
  })
}
