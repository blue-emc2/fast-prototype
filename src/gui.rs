use iced::TextInput;
use iced::{executor, text_input, Application, Column, Command, Element, Text};

pub struct GUI {
  state2: text_input::State,
}

#[derive(Debug, Clone)]
pub enum Message {
  TextInputChanged(String),
}

impl Application for GUI {
  type Executor = executor::Default;
  type Message = Message;
  type Flags = ();

  fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
    (
      GUI {
        state2: text_input::State::new(),
      },
      Command::none(),
    )
  }

  fn title(&self) -> String {
    String::from("prototype")
  }

  fn update(&mut self, _message: Self::Message, _: &mut iced::Clipboard) -> Command<Self::Message> {
    Command::none()
  }

  fn view(&mut self) -> Element<Self::Message> {
    let input = TextInput::new(
      &mut self.state2,
      "This is the placeholder...",
      "bbb",
      Message::TextInputChanged,
    );

    Column::new()
      .push(Text::new("Hello, World!"))
      .push(input)
      .into()
  }
}
