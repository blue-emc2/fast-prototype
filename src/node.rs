#[derive(Debug, Clone, Copy)]
pub struct Node {
  node_type: NodeState,
}

#[derive(Debug, Clone, Copy)]
pub enum NodeState {
  Init,
  Action,
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
