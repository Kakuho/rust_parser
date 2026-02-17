use super::expressions;

pub struct CstLetStatement{
  // handle outer attributes later
  // handle PatternNoTopAlt later
  pub let_type: Option<String>,                // type is a string, semantic analsyer phase can handle validity of types
  pub expression: Option<expressions::CstExpression> 
}
