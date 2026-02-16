use crate::{concrete::cst::CstFunctionQualifier, lex};
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

  pub fn parse_function(&mut self) -> Option<cst::CstFunction>{
    // for now just matches the braces
    match self.tokens[self.position]{
      lex::LexerToken::LeftCurlyBrace => self.position = self.position+1,
      _ => panic!("damn bro")
    };
    match self.tokens[self.position]{
      lex::LexerToken::RightCurlyBrace => self.position = self.position+1,
      _ => panic!("damn bro")
    
    };
    panic!("CANNOT PARSE FUNCTIONS YET")
    return None;
  }

  fn parse_function_qualifiers(&mut self) -> cst::CstFunctionQualifier{
    // doesn't handle extern yet
    let is_const = match self.tokens[self.position]{
      lex::LexerToken::Const => {
        self.position += 1;
        true
      },
      _ => false
    }

    let is_async = match self.tokens[self.position]{
      lex::LexerToken::Async => {
        self.position += 1;
        true
      },
      _ => false
    }
  
    let is_unsafe: bool;
    let is_safe: bool;
  
    match self.tokens[self.position]{
      lex::LexerToken::Unsafe => {
        self.position += 1;
        is_unsafe = true;
        is_safe = false;
      },
      lex::LexerToken::Safe =>{
        self.position += 1;
        is_safe = true;
        is_unsafe = false;
      }
      _ => {
        is_safe = true;
        is_unsafe = false;
      }
    }

    return cst::CstFunctionQualifier{
      is_const: is_const,
      is_async: is_async,
      is_safe: is_safe,
      is_unsafe: is_unsafe
    }
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
