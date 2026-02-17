use super::expressions;

pub struct CstLetStatement{
  // handle outer attributes later
  // handle PatternNoTopAlt later
  let_type: Option<String>,                // type is a string, semantic analsyer phase can handle validity of types
  expression: Option<expressions::CstExpression> 
}
