use super::expressions;

pub enum PatternNoTopAltKind{
  PatternWithoutRange(CstPatternWithoutRange),
  //RangePattern(CstRangePattern)  later...
}

pub struct CstPatternNoTopAlt{
  kind: PatternWithoutRangeKind
}

enum PatternWithoutRangeKind{
  LiteralPattern(expressions::CstLiteralExpression),
  IdentifierPattern(String)
}

pub struct CstPatternWithoutRange{
  kind: PatternWithoutRangeKind
}

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

