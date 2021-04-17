use iced::Application;
use iced::Settings;

mod gui;
use gui::GUI;

pub fn main() -> iced::Result {
  GUI::run(Settings::default())
}
