use iced::{
  canvas::{Cursor, Frame, Geometry, LineCap, LineJoin, Path, Program, Stroke},
  Color, Point, Rectangle, Size,
};

use crate::gui::Message;

#[derive(Debug)]
pub struct Node {}

impl Program<Message> for Node {
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
