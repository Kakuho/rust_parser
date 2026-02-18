use super::expressions;
use super::patterns;

pub struct CstLetStatement{
  // handle outer attributes later
  // handle PatternNoTopAlt later
  pub pattern: Option<patterns::CstPatternNoTopAlt>,
  pub let_type: Option<String>,                // type is a string, semantic analsyer phase can handle validity of types
  pub expression: Option<expressions::CstExpression> 
}

impl CstLetStatement{
  pub fn Print(&self){
    println!("Let Statement");

    print!("let Type: ");

    match &self.let_type{
      None => println!("no type annotation provided"),
      Some(let_type) => println!("type: {}", &let_type)
    }

    match &self.pattern{
      Some(pattern) => pattern.Print(),
      None => println!("no pattern provided"),
    }

    match &self.expression{
      Some(expression) => expression.Print(),
      None => println!("no expression provided"),
    
    }
  }
}
