use crate::node::NodeState;

pub struct Lexer {}

impl Lexer {
  pub fn scan(token: &str) -> Result<Vec<NodeState>, String> {
    let chars = token.replace(" ", "").chars().collect::<Vec<char>>();

    if chars.len() == 1 {
      return Ok(vec![NodeState::Action]);
    }

    match chars[1] {
      '?' => Ok(vec![NodeState::Decision]),
      _ => Ok(vec![NodeState::Action]),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_node_state_action_from_hoge() {
    let state = Lexer::scan("hoge");
    assert_eq!(state, Ok(vec![NodeState::Action]));
  }

  #[test]
  fn test_node_state_decision_from_a_b_c() {
    let state = Lexer::scan("a ? b : c");
    assert_eq!(state, Ok(vec![NodeState::Decision]));
  }

  #[test]
  fn test_node_state_action_from_a() {
    let state = Lexer::scan("A");
    assert_eq!(state, Ok(vec![NodeState::Action]));
  }

  #[test]
  fn test_node_state_decision_from_abc_d_e() {
    let state = Lexer::scan("abc ? d : e");
    assert_eq!(state, Ok(vec![NodeState::Decision]));
  }
}
