#[derive(Debug, Clone, Copy)]
pub struct Node {
  node_type: NodeState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeState {
  Init,
  Action,
  None,
  Decision,
}

impl Node {
  pub fn new(node_type: NodeState) -> Node {
    Node {
      node_type: node_type,
    }
  }

  pub fn node_type(self) -> NodeState {
    self.node_type
  }
}
