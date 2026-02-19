use super::expressions;
use super::patterns;

pub struct CstStatements{
  // actually needs a collection this time hehe
  statements: Vec<CstStatement>
}

impl CstStatements{
  pub fn make_empty() -> Self{
    CstStatements{statements: Vec::new()}
  }

  pub fn add_statement(&mut self, stmt: CstStatement){
    self.statements.push(stmt);
  }

  pub fn len(&self) -> usize{
    self.statements.len()
  }

  pub fn print(&self){
    println!("Statements");
    for statement in &self.statements{
      statement.print();
    }
  }
}

pub enum StatementKind{
  LetStatement(CstLetStatement)
}

impl StatementKind{
  pub fn print(&self){
    match &self{
      StatementKind::LetStatement(stmt) => {stmt.print();}
    }
  }
}

pub struct CstStatement{
  kind: StatementKind
}

impl CstStatement{
  pub fn print(&self){
    println!("Statement");
    self.kind.print();
  }
}

impl From<CstLetStatement> for CstStatement{
  fn from(node: CstLetStatement) -> Self{
    return CstStatement{kind: StatementKind::LetStatement(node)};
  }
}

pub struct CstLetStatement{
  // handle outer attributes later
  // handle PatternNoTopAlt later
  pub pattern: Option<patterns::CstPatternNoTopAlt>,
  pub let_type: Option<String>,                // type is a string, semantic analsyer phase can handle validity of types
  pub expression: Option<expressions::CstExpression> 
}

impl CstLetStatement{
  pub fn print(&self){
    println!("Let Statement");

    print!("let Type: ");

    match &self.let_type{
      None => println!("no type annotation provided"),
      Some(let_type) => println!("type: {}", &let_type)
    }

    match &self.pattern{
      Some(pattern) => pattern.print(),
      None => println!("no pattern provided"),
    }

    match &self.expression{
      Some(expression) => expression.print(),
      None => println!("no expression provided"),
    
    }
  }
}
