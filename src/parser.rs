use crate::node::NodeState;

pub struct Parser {}

impl Parser {
  pub fn parse(_token: &str) -> Result<NodeState, String> {
    Ok(NodeState::Action)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_node_state_action() {
    let state = Parser::parse("hoge");
    assert_eq!(state, Ok(NodeState::Action));
  }

  #[test]
  fn test_node_state_decision_from_token() {
    let state = Parser::parse("a ? b : c");
    assert_eq!(state, Ok(NodeState::Decision));
  }
}
