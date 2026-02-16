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
  LeftCurlyBrace,
  RightCurlyBrace,
  LeftRoundBrace,
  RightRoundBrace,
  Identifier(String)
}
