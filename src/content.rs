use iced::Container;
use iced::{
  canvas::{Canvas, Cursor, Frame, Geometry, Path, Program},
  text_input, Color, Element, Length, Rectangle, TextInput,
};

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
    let canvas = Canvas::new(Circle { radius: 50.0 });

    match self.position {
      ContentState::Left => Container::new(input)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(5)
        .into(),
      ContentState::Right => Container::new(canvas)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .center_x()
        .center_y()
        .into(),
    }
  }
}

#[derive(Debug)]
struct Circle {
  radius: f32,
}

// Then, we implement the `Program` trait
impl Program<Message> for Circle {
  fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
    // We prepare a new `Frame`
    let mut frame = Frame::new(bounds.size());

    // We create a `Path` representing a simple circle
    let circle = Path::circle(frame.center(), self.radius);

    // And fill it with some color
    frame.fill(&circle, Color::BLACK);

    // Finally, we produce the geometry
    vec![frame.into_geometry()]
  }
}
