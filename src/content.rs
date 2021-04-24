use crate::node::Node;
use iced::{canvas::Canvas, text_input, Container, Element, Length, TextInput};

use crate::gui::Message;

#[derive(Debug, Clone)]
pub enum ContentState {
  Left,
  Right,
}

pub struct Content {
  input_state: text_input::State,
  position: ContentState,
}

impl Content {
  pub fn new(position: ContentState) -> Self {
    Content {
      input_state: text_input::State::new(),
      position: position,
    }
  }

  pub fn view(&mut self) -> Element<Message> {
    let input = TextInput::new(
      &mut self.input_state,
      "This is the placeholder...",
      "bbb",
      Message::TextInputChanged,
    )
    .padding(10);
    let canvas = Canvas::new(Node {})
      .width(Length::Fill)
      .height(Length::Fill);

    match self.position {
      ContentState::Left => Container::new(input)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(5)
        .into(),
      ContentState::Right => Container::new(canvas)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(5)
        .into(),
    }
  }
}
