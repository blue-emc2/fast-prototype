use crate::node::NodeState;

pub struct Lexer {}

impl Lexer {
  pub fn scan(input: &str) -> Result<Vec<NodeState>, String> {
    let mut result = Vec::new();
    let token_iter = input.split_whitespace();
    for token in token_iter {
      let state = match token {
        "?" => NodeState::Decision,
        ":" => NodeState::None,
        _ => NodeState::Action,
      };

      result.push(state);
    }

    Ok(result)
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
    assert_eq!(
      state,
      Ok(vec![
        NodeState::Action,
        NodeState::Decision,
        NodeState::Action,
        NodeState::None,
        NodeState::Action,
      ])
    );
  }

  #[test]
  fn test_node_state_action_from_a() {
    let state = Lexer::scan("A");
    assert_eq!(state, Ok(vec![NodeState::Action]));
  }

  #[test]
  fn test_node_state_decision_from_abc_d_e() {
    let state = Lexer::scan("abc ? d : e");
    assert_eq!(
      state,
      Ok(vec![
        NodeState::Action,
        NodeState::Decision,
        NodeState::Action,
        NodeState::None,
        NodeState::Action,
      ])
    );
  }
}
