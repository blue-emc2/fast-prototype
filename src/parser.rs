use crate::node::NodeState;

pub struct Parser {}

impl Parser {
  pub fn parse(token: &str) -> Result<NodeState, String> {
    let chars = token.replace(" ", "").chars().collect::<Vec<char>>();

    if chars.len() == 1 {
      return Ok(NodeState::Action);
    }

    match chars[1] {
      '?' => Ok(NodeState::Decision),
      _ => Ok(NodeState::Action),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_node_state_action_from_hoge() {
    let state = Parser::parse("hoge");
    assert_eq!(state, Ok(NodeState::Action));
  }

  #[test]
  fn test_node_state_decision_from_a_b_c() {
    let state = Parser::parse("a ? b : c");
    assert_eq!(state, Ok(NodeState::Decision));
  }

  #[test]
  fn test_node_state_action_from_a() {
    let state = Parser::parse("A");
    assert_eq!(state, Ok(NodeState::Action));
  }
}
