use crate::gui::Message;
use iced::{button, Align, Button, Element, Row, Text};

#[derive(Debug, Clone)]
pub struct Node {
  pub node_type: NodeType,
  pub text_input: String,
  state: NodeState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
  Init,
  Action,
  None,
  Decision,
}

#[derive(Debug, Clone, Copy)]
pub enum NodeState {
  Idle { edit_button: button::State },
}

impl Default for NodeState {
  fn default() -> Self {
    NodeState::Idle {
      edit_button: button::State::new(),
    }
  }
}

impl Node {
  pub fn new(node_type: NodeType, text_input: String) -> Node {
    Node {
      node_type: node_type,
      state: NodeState::Idle {
        edit_button: button::State::new(),
      },
      text_input: String::from(text_input),
    }
  }

  pub fn view(&mut self) -> Element<Message> {
    match &mut self.state {
      NodeState::Idle { edit_button } => {
        let row = Row::new()
          .align_items(Align::Center)
          .push(Text::new(self.text_input.clone()))
          .push(Button::new(edit_button, Text::new("Press me!")));
        row.into()
      }
    }
  }
}
