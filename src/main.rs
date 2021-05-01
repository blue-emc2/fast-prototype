use iced::{Application, Settings};

mod content;
mod flowchart;
mod gui;
mod node;

use gui::GUI;

pub fn main() -> iced::Result {
  GUI::run(Settings {
    antialiasing: true,
    ..Settings::default()
  })
}
