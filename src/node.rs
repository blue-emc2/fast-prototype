use iced::button;

#[derive(Debug, Clone, Copy)]
pub struct Node {
  node_type: NodeType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
  Init,
  Action,
  None,
  Decision,
}

pub enum NodeState {
  Idle { edit_button: button::State },
}

impl Node {
  pub fn new(node_type: NodeType) -> Node {
    Node {
      node_type: node_type,
    }
  }

  pub fn node_type(self) -> NodeType {
    self.node_type
  }
}
