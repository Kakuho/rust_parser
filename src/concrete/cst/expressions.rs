pub struct CstBlockExpression{
  // not parsing attributes for now, later will add attribute parsing
  // statements: Option<CstStatements>
}

impl CstBlockExpression{
  pub fn Print(&self){
    println!("Block Expression");
  }
}

pub enum ExpressionKind{
  LiteralExpression(CstLiteralExpression),
}

pub struct CstExpression{
  // this maps to expression_without_block
  pub kind: ExpressionKind
}

///////////////////////////////////////////////////// literal expressions
// its kinda annoying that i have to do a long /// in order to explicity say "here's a piece of code that implements XYZ"...

enum LiteralExpressionVal{
  Char(char),
  String(String),
  Integral(i32),
  Float(f64)
}

pub struct CstLiteralExpression{
  literal: LiteralExpressionVal
}

impl From<char> for CstLiteralExpression{
  fn from(item: char) -> Self{
    return CstLiteralExpression {
      literal: LiteralExpressionVal::Char(item)
    }
  }
}

impl From<String> for CstLiteralExpression{
  fn from(item: String) -> Self{
    return CstLiteralExpression {
      literal: LiteralExpressionVal::String(item)
    }
  }
}

impl From<i32> for CstLiteralExpression{
  fn from(item: i32) -> Self{
    return CstLiteralExpression {
      literal: LiteralExpressionVal::Integral(item)
    }
  }
}

impl From<f64> for CstLiteralExpression{
  fn from(item: f64) -> Self{
    return CstLiteralExpression {
      literal: LiteralExpressionVal::Float(item)
    }
  }
}
