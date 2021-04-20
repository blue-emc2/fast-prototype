use iced::{
  executor, pane_grid, text_input, Application, Command, Container, Element, Length, PaneGrid,
  Text, TextInput,
};

pub struct GUI {
  state2: text_input::State,
  panes: pane_grid::State<PaneState>,
}

#[derive(Debug, Clone)]
enum PaneState {
  Init,
  Init2,
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
      a: Box::new(pane_grid::Configuration::Pane(PaneState::Init)),
      b: Box::new(pane_grid::Configuration::Pane(PaneState::Init2)),
    });

    (
      GUI {
        state2: text_input::State::new(),
        panes: state,
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
    let _input = TextInput::new(
      &mut self.state2,
      "This is the placeholder...",
      "bbb",
      Message::TextInputChanged,
    );

    let pane_grid = PaneGrid::new(&mut self.panes, |_pane, content| {
      pane_grid::Content::new(match content {
        PaneState::Init => Text::new("Init"),
        PaneState::Init2 => Text::new("Init2"),
      })
    });

    Container::new(pane_grid)
      .width(Length::Fill)
      .height(Length::Fill)
      .padding(10)
      .into()
  }
}
