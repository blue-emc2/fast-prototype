use iced::{executor, Application, Command, Element, Text};

pub struct GUI {}

impl Application for GUI {
  type Executor = executor::Default;
  type Message = ();
  type Flags = ();

  fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
    (GUI {}, Command::none())
  }

  fn title(&self) -> String {
    String::from("prototype")
  }

  fn update(&mut self, _message: Self::Message, _: &mut iced::Clipboard) -> Command<Self::Message> {
    Command::none()
  }

  fn view(&mut self) -> Element<Self::Message> {
    Text::new("Hello, World!").into()
  }
}
