pub struct Lexer{
  position: usize
}

pub enum LexerToken{
  Char(char),
  // keywords, could be stored in a  hashmap instead 
  Fn,
  Const,
  Async,
  Safe,
  Unsafe,
  Extern,
  Let,
  Abi(String),
  // literals
  Character(char),
  String(String),
  Integral(i32),
  Float(f64),
  // others
  Type(String),
  // symbols
  SingleQuote,
  DoubleQuote,
  Colon,
  Equal,
  SemiColon,
  // braces
  LeftCurlyBrace,
  RightCurlyBrace,
  LeftRoundBrace,
  RightRoundBrace,
  // values
  Identifier(String)
}
