use super::expressions;

pub enum PatternNoTopAltKind{
  PatternWithoutRange(CstPatternWithoutRange),
  //RangePattern(CstRangePattern)  later...
}

pub struct CstPatternNoTopAlt{
  kind: PatternNoTopAltKind
}

impl From<CstPatternWithoutRange> for CstPatternNoTopAlt{
  fn from(node: CstPatternWithoutRange) -> Self{
    CstPatternNoTopAlt{
      kind: PatternNoTopAltKind::PatternWithoutRange(node)
    }
  }
}

impl CstPatternNoTopAlt{
  pub fn print(&self){
    println!("CstPatternNoTopAlt");
    match &self.kind{
      PatternNoTopAltKind::PatternWithoutRange(node) => {node.print();}
    }
  }
}

////////////////////////////////////////////////////////

enum PatternWithoutRangeKind{
  LiteralPattern(expressions::CstLiteralExpression),
  IdentifierPattern(CstIdentifierPattern)
}

pub struct CstPatternWithoutRange{
  kind: PatternWithoutRangeKind
}

impl From<CstIdentifierPattern> for CstPatternWithoutRange{
  fn from(node: CstIdentifierPattern) -> Self{
    CstPatternWithoutRange{ kind: PatternWithoutRangeKind::IdentifierPattern(node)}
  }
}

impl CstPatternWithoutRange{
  pub fn print(&self){
    println!("PatternWithoutRange");
    match &self.kind{
      PatternWithoutRangeKind::IdentifierPattern(node) => {node.print();}
      _ => {}
    }
  }
}


////////////////////////////////////////////////////////

pub struct CstIdentifierPattern{
  is_ref: bool,
  is_mutable: bool,
  identifier: String
}

impl CstIdentifierPattern{
  pub fn create(is_ref: bool, is_mutable: bool, identifier: String) -> Self{
    Self{
      is_ref: is_ref,
      is_mutable: is_mutable,
      identifier: identifier
    }
  }

  pub fn print(&self){
    println!("Identifier Pattern");
    if self.is_ref{
      println!("Is Reference")
    }
    if self.is_mutable{
      println!("Is Mutable")
    }
    println!("Identifier: {}", self.identifier);
  }
}

