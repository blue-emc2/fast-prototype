use iced::{executor, pane_grid, Application, Command, Container, Element, Length, PaneGrid};

use crate::content::Content;
use crate::content::ContentState;

pub struct GUI {
  panes: pane_grid::State<Content>,
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

    (GUI { panes: state }, Command::none())
  }

  fn title(&self) -> String {
    String::from("prototype")
  }

  fn update(&mut self, _message: Self::Message, _: &mut iced::Clipboard) -> Command<Self::Message> {
    Command::none()
  }

  fn view(&mut self) -> Element<Self::Message> {
    let pane_grid = PaneGrid::new(&mut self.panes, |_pane, content| {
      pane_grid::Content::new(content.view()).style(style::Pane)
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
