use iced::{
  canvas::{Cursor, Frame, Geometry, LineCap, LineJoin, Path, Program, Stroke, Text},
  Color, Point, Rectangle, Size,
};

use crate::gui::Message;
use crate::node::Node;
use crate::node::NodeState;

const SIZE_WIDTH: f32 = 150.0;
const SIZE_HEIGHT: f32 = 74.0;
const RADIUS: f32 = 15.0;
const INIT_POS: f32 = 40.0;

#[derive(Debug)]
pub struct FlowChart {
  diagrams: Vec<Diagram>,
  diagram_created: usize,
}

impl FlowChart {
  pub fn push_node(&mut self, node: &Node, content: &str) {
    let mut diagram = Diagram::from((node, content));
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

    // ノードを結ぶ線を引く
    // let line = Path::line(
    //   Point {
    //     x: center.x,
    //     y: 0.0,
    //   },
    //   center,
    // );
    // frame.stroke(
    //   &line,
    //   Stroke {
    //     ..Default::default()
    //   },
    // );

    // ノードを描画する
    for d in self.diagrams.iter() {
      let mut init_pos = Point {
        x: center.x,
        y: INIT_POS,
      };

      match d.node_type {
        NodeState::Init => {
          let circle = Path::circle(init_pos, RADIUS);
          frame.fill(&circle, Color::BLACK);
        }
        NodeState::Action => {
          init_pos.x = init_pos.x - (d.size.width / 2.0); // 四角の中心とpaneの中心をあわせる
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

          frame.fill_text(Text {
            content: d.content,
            position: Point {
              x: init_pos.x,
              y: init_pos.y,
            },
            ..Default::default()
          });
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
  content: String,
}

impl From<(&Node, &str)> for Diagram {
  fn from(tuple: (&Node, &str)) -> Diagram {
    Diagram {
      node_type: tuple.0.node_type(),
      size: Size::new(SIZE_WIDTH, SIZE_HEIGHT),
      index: 0,
      content: tuple.1.to_string(),
    }
  }
}
