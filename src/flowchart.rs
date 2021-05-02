use iced::{
  canvas::{Cursor, Frame, Geometry, LineCap, LineJoin, Path, Program, Stroke},
  Color, Point, Rectangle, Size,
};

use crate::gui::Message;
use crate::node::Node;
use crate::node::NodeState;

const SIZE_WIDTH: f32 = 150.0;
const SIZE_HEIGHT: f32 = 74.0;
const RADIUS: f32 = 15.0;

#[derive(Debug)]
pub struct FlowChart {
  diagrams: Vec<Diagram>,
  diagram_created: usize,
}

impl FlowChart {
  pub fn push_node(&mut self, node: Node) {
    let mut diagram = Diagram::from(node);
    diagram.index = self.diagram_created;
    self.diagrams.push(diagram);
    self.diagram_created += 1;
  }
}

impl Default for FlowChart {
  fn default() -> Self {
    Self {
      diagrams: Vec::new(),
      diagram_created: 0,
    }
  }
}

impl Program<Message> for FlowChart {
  fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
    let mut frame = Frame::new(bounds.size());
    let center = frame.center();

    for d in self.diagrams.iter() {
      let mut init_pos = Point {
        x: center.x,
        y: 40.0,
      };

      match d.node_type {
        NodeState::Init => {
          let circle = Path::circle(init_pos, RADIUS);
          frame.fill(&circle, Color::BLACK);
        }
        NodeState::Action => {
          init_pos.y = (init_pos.y + d.size.height) * d.index as f32;
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
        }
      }
    }

    vec![frame.into_geometry()]
  }
}

#[derive(Debug)]
struct Diagram {
  node_type: NodeState,
  size: Size,
  index: usize,
}

impl From<Node> for Diagram {
  fn from(node: Node) -> Diagram {
    Diagram {
      node_type: node.node_type(),
      size: Size::new(SIZE_WIDTH, SIZE_HEIGHT),
      index: 0,
    }
  }
}
