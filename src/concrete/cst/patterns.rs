use super::expressions;

enum PatternNoTopAltKind{
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

pub struct IdentifierPattern{
  is_ref: bool,
  is_mutable: bool,
  identifier: String
}
