use iced::{
  executor, pane_grid, text_input, Application, Command, Container, Element, Length, PaneGrid,
  Text, TextInput,
};

pub struct GUI {
  state2: text_input::State,
  panes: pane_grid::State<Content>,
}

enum Content {
  Split,
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
    let (panes, _) = pane_grid::State::new(Content::Split);

    (
      GUI {
        state2: text_input::State::new(),
        panes: panes,
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
    let input = TextInput::new(
      &mut self.state2,
      "This is the placeholder...",
      "bbb",
      Message::TextInputChanged,
    );

    let pane_grid = PaneGrid::new(&mut self.panes, |pane, content| {
      pane_grid::Content::new(Text::new("This is some pane"))
    });

    Container::new(pane_grid)
      .width(Length::Fill)
      .height(Length::Fill)
      .padding(10)
      .into()
  }
}
