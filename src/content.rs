use crate::node::Node;
use iced::{
  canvas::{Canvas, Cursor, Frame, Geometry, LineCap, LineJoin, Path, Program, Stroke},
  pane_grid::Pane,
  text_input, Color, Container, Element, Length, Point, Rectangle, Size, TextInput,
};

use crate::gui::Message;

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
}

impl Content {
  pub fn new(position: ContentState) -> Self {
    Content {
      input_state: text_input::State::new(),
      position: position,
      input_value: String::from(""),
    }
  }

  pub fn view(&mut self, _pane: Pane, nodes: &Vec<Node>) -> Element<Message> {
    let position = self.position;
    let diagram = Diagram {
      state: DiagramState {
        nodes: nodes.to_vec(),
      },
    };

    let canvas = Canvas::new(diagram)
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

    match position {
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

  pub fn update_text_input(&mut self, value: String) {
    self.input_value = value;
  }
}

const SIZE_WIDTH: f32 = 150.0;
const SIZE_HEIGHT: f32 = 74.0;

#[derive(Debug)]
struct DiagramState {
  nodes: Vec<Node>,
}

#[derive(Debug)]
struct Diagram {
  state: DiagramState,
}

impl Program<Message> for Diagram {
  fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
    let mut frame = Frame::new(bounds.size());
    let point = bounds.size();
    let size = Size::new(SIZE_WIDTH, SIZE_HEIGHT);
    let x = (point.width / 2.0) - (size.width / 2.0);
    let mut offset = 0.0;
    let padding = 10.0;
    let mut last_pos_y = (SIZE_HEIGHT * offset) + padding;

    for _node in &self.state.nodes {
      let init_pos = Point {
        x: x,
        y: last_pos_y,
      };

      let rect = Path::rectangle(init_pos, size);
      frame.stroke(
        &rect,
        Stroke {
          color: Color::BLACK,
          width: 1.0,
          line_cap: LineCap::Round,
          line_join: LineJoin::Round,
        },
      );

      offset += 1.0;
      last_pos_y += (SIZE_HEIGHT * offset) + padding;
    }

    vec![frame.into_geometry()]
  }
}
