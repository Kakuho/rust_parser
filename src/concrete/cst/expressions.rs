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

enum LiteralExpressionVal{
  Char(char),
  String(String),
  Integral(isize),
  Float(f64)
}

pub struct CstLiteralExpression{
  literal: LiteralExpressionVal
}

