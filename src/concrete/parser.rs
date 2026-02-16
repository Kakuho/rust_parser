use crate::lex;
use super::cst;

pub struct Parser{
  position: usize,
  tokens: Vec<lex::LexerToken>
}

impl Parser{

  pub fn create(srctokens: Vec<lex::LexerToken>) -> Parser{
    Parser{
      position: 0,
      tokens: srctokens
    }
  }

  pub fn parse(self) -> i8{
    // move the lexer into this function call and kill it self
    2
  }

  pub fn parse_block_expression(&mut self) -> Option<cst::CstBlockExpression>{
    // for now just matches the braces
    match self.tokens[self.position]{
      lex::LexerToken::LeftCurlyBrace => self.position = self.position+1,
      _ => panic!("damn bro")
    };
    match self.tokens[self.position]{
      lex::LexerToken::RightCurlyBrace => self.position = self.position+1,
      _ => panic!("damn bro")
    };
    return Some(cst::CstBlockExpression{});
  }
 
}
