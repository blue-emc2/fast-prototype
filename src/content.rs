use crate::node::Node;
use iced::{
  canvas::{Canvas, Cursor, Frame, Geometry, LineCap, LineJoin, Path, Program, Stroke},
  text_input, Color, Container, Element, Length, Point, Rectangle, Size, TextInput,
};

use crate::gui::Message;

#[derive(Debug, Clone, Copy)]
pub enum ContentState {
  Left,
  Right,
}

pub struct Content {
  input_state: text_input::State,
  position: ContentState,
  diagram: Diagram,
}

impl Content {
  pub fn new(position: ContentState) -> Self {
    Content {
      input_state: text_input::State::new(),
      position: position,
      diagram: Diagram {},
    }
  }

  pub fn view(&mut self, _nodes: &Vec<Node>) -> Element<Message> {
    let position = self.position;
    let canvas = Canvas::new(self.diagram)
      .width(Length::Fill)
      .height(Length::Fill);
    let input = TextInput::new(
      &mut self.input_state,
      "This is the placeholder...",
      "bbb",
      Message::TextInputChanged,
    )
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
}

#[derive(Debug, Clone, Copy)]
struct Diagram {}

impl Program<Message> for Diagram {
  fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
    let mut frame = Frame::new(bounds.size());
    let size = Size::new(200.0, 124.0);

    let point = bounds.size();
    let init_pos = Point {
      x: (point.width / 2.0) - (size.width / 2.0),
      y: 10.0,
    };
    let rect = Path::rectangle(init_pos, size);

    frame.stroke(
      &rect,
      Stroke {
        color: Color::BLACK,
        width: 2.0,
        line_cap: LineCap::Round,
        line_join: LineJoin::Round,
      },
    );

    vec![frame.into_geometry()]
  }
}
