#[derive(Debug, Clone)]
pub struct Node {
  node_type: NodeState,
}

#[derive(Debug, Clone)]
pub enum NodeState {
  Action,
}

impl Node {
  pub fn new(node_type: NodeState) -> Node {
    Node {
      node_type: node_type,
    }
  }
}
