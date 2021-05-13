use crate::node::Node;
use iced::{
  canvas::Canvas, pane_grid::Pane, text_input, Align, Column, Container, Element, Length, Text,
  TextInput,
};

use crate::flowchart::FlowChart;
use crate::gui::Message;
use crate::lexer::Lexer;
use crate::node::NodeType;

#[derive(Debug, Clone, Copy)]
pub enum ContentState {
  Left,
  Right,
}

#[derive(Debug)]
pub struct Content {
  input_state: text_input::State,
  position: ContentState,
  input_value: String,
  nodes: Vec<Node>,
  flowchart: FlowChart,
}

impl Content {
  pub fn new(position: ContentState) -> Self {
    let start_node = Node::new(NodeType::Init);
    let nodes = vec![start_node];
    let mut flowchart: FlowChart = Default::default();
    flowchart.push_node(&start_node, &String::new());

    Content {
      input_state: text_input::State::new(),
      position: position,
      input_value: String::from(""),
      nodes: nodes,
      flowchart: flowchart,
    }
  }

  pub fn view(&mut self, _pane: Pane) -> Element<Message> {
    let position = self.position;
    let canvas = Canvas::new(&mut self.flowchart)
      .width(Length::Fill)
      .height(Length::Fill);
    let input = TextInput::new(
      &mut self.input_state,
      "This is the placeholder...",
      &self.input_value,
      Message::TextInputChanged,
    )
    .on_submit(Message::CreateNode)
    .padding(10);

    let mut children = vec![input.into()];
    for _node in self.nodes.iter() {
      children.push(Text::new("hoge").into())
    }
    let column = Column::with_children(children)
      .width(Length::Fill)
      .align_items(Align::Center);

    match position {
      ContentState::Left => Container::new(column)
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

  pub fn update(&mut self, message: &Message) {
    match message {
      Message::TextInputChanged(value) => {
        self.input_value = value.to_string();
      }
      Message::CreateNode => {
        if !self.input_value.is_empty() {
          let result = Lexer::scan(&self.input_value);

          match result {
            Ok(nodes) => {
              for node in nodes {
                let node = Node::new(node);
                self.nodes.push(node);
                self.flowchart.push_node(&node, &self.input_value);
              }
              self.input_value.clear();
            }
            Err(_e) => println!("error"),
          }
        }
      }
    }
  }
}
