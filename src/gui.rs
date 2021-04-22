use iced::Column;
use iced::{
  executor, pane_grid, text_input, Application, Command, Container, Element, Length, PaneGrid,
  Text, TextInput,
};

pub struct GUI {
  panes: pane_grid::State<Content>,
}

#[derive(Debug, Clone)]
enum PaneState {
  LeftPaneInit,
  RightPaneInit,
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
      a: Box::new(pane_grid::Configuration::Pane(Content::new())),
      b: Box::new(pane_grid::Configuration::Pane(Content::new())),
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
      pane_grid::Content::new(content.view())
    });

    Container::new(pane_grid)
      .width(Length::Fill)
      .height(Length::Fill)
      .padding(10)
      .into()
  }
}

struct Content {
  input_state: text_input::State,
}

impl Content {
  fn new() -> Self {
    Content {
      input_state: text_input::State::new(),
    }
  }

  fn view(&mut self) -> Element<Message> {
    let input = TextInput::new(
      &mut self.input_state,
      "This is the placeholder...",
      "bbb",
      Message::TextInputChanged,
    );

    Column::new().push(input).into()
  }
}
