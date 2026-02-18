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
}

