use iced::{
  canvas::{Cursor, Frame, Geometry, LineCap, LineJoin, Path, Program, Stroke},
  Color, Point, Rectangle, Size,
};

use crate::gui::Message;
use crate::node::Node;
use crate::node::NodeState;

const SIZE_WIDTH: f32 = 150.0;
const SIZE_HEIGHT: f32 = 74.0;

#[derive(Debug)]
pub struct FlowChart {
  diagrams: Vec<Diagram>,
}

impl FlowChart {
  pub fn push_node(&mut self, node: Node) {
    let diagram = Diagram::from(node);
    self.diagrams.push(diagram);
  }
}

impl Default for FlowChart {
  fn default() -> Self {
    Self {
      diagrams: Vec::new(),
    }
  }
}

impl Program<Message> for FlowChart {
  fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
    let mut frame = Frame::new(bounds.size());
    let point = bounds.size();
    let mut offset = 0.0;
    let padding = 10.0;
    let mut last_pos_y = (SIZE_HEIGHT * offset) + padding;

    for d in self.diagrams.iter() {
      let init_pos = Point {
        x: (point.width / 2.0) - (d.size.width / 2.0),
        y: last_pos_y,
      };

      let rect = Path::rectangle(init_pos, d.size);
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

#[derive(Debug)]
struct Diagram {
  node_type: NodeState,
  size: Size,
}

impl From<Node> for Diagram {
  fn from(node: Node) -> Diagram {
    Diagram {
      node_type: node.node_type(),
      size: Size::new(SIZE_WIDTH, SIZE_HEIGHT),
    }
  }
}
