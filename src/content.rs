use iced::{text_input, Column, Text, TextInput};

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

  pub fn view(&mut self) -> Column<Message> {
    let input = TextInput::new(
      &mut self.input_state,
      "This is the placeholder...",
      "bbb",
      Message::TextInputChanged,
    );

    match self.position {
      ContentState::Left => Column::new().push(input).into(),
      ContentState::Right => Column::new().push(Text::new("I <3 iced!")).into(),
    }
  }
}
