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
  Integer(i32),
  Character(char),
  // symbols
  SingleQuote,
  DoubleQuote,
  // braces
  LeftCurlyBrace,
  RightCurlyBrace,
  LeftRoundBrace,
  RightRoundBrace,
  // values
  Identifier(String)
}
