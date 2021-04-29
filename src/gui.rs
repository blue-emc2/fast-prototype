use iced::{executor, pane_grid, Application, Command, Container, Element, Length, PaneGrid};

use crate::content::Content;
use crate::content::ContentState;
use crate::node::Node;
use crate::node::NodeState;

pub struct GUI {
  panes: pane_grid::State<Content>,
  nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Message {
  TextInputChanged(String),
}

impl Application for GUI {
  type Executor = executor::Default;
  type Message = Message;
  type Flags = ();

  fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
    let state = pane_grid::State::with_configuration(pane_grid::Configuration::Split {
      axis: pane_grid::Axis::Vertical,
      ratio: 0.5,
      a: Box::new(pane_grid::Configuration::Pane(Content::new(
        ContentState::Left,
      ))),
      b: Box::new(pane_grid::Configuration::Pane(Content::new(
        ContentState::Right,
      ))),
    });

    // let mut init_node = Node { type: NodeState::Init };
    let init_node = Node::new(NodeState::Action);
    let mut nodes = Vec::with_capacity(1);
    nodes.push(init_node);

    (
      GUI {
        panes: state,
        nodes: nodes,
      },
      Command::none(),
    )
  }

  fn title(&self) -> String {
    String::from("prototype")
  }

  fn update(&mut self, _message: Self::Message, _: &mut iced::Clipboard) -> Command<Self::Message> {
    Command::none()
  }

  fn view(&mut self) -> Element<Self::Message> {
    let nodes = &self.nodes;

    let pane_grid = PaneGrid::new(&mut self.panes, |pane, content| {
      pane_grid::Content::new(content.view(pane, nodes)).style(style::Pane)
    })
    .spacing(10);

    Container::new(pane_grid)
      .width(Length::Fill)
      .height(Length::Fill)
      .padding(5)
      .into()
  }
}

mod style {
  use iced::{container, Color};

  pub struct Pane;

  impl container::StyleSheet for Pane {
    fn style(&self) -> container::Style {
      container::Style {
        border_width: 2.0,
        border_color: Color::BLACK,
        border_radius: 5.0,
        ..Default::default()
      }
    }
  }
}
