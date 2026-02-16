use std::ops::Deref;

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
    let qualifiers = self.parse_function_qualifiers();

    match self.tokens[self.position]{
      lex::LexerToken::Fn => {
        self.position += 1;
      },
      _ => panic!("Failed to parse function")
    }

    let ref ident_token = self.tokens[self.position];

    match ident_token{
      lex::LexerToken::Identifier(ident) => {
        self.position += 1;
      },
      _ => panic!("Failed to parse function")
    }

    // skip the generic parameters for now

    match self.tokens[self.position]{
      lex::LexerToken::LeftRoundBrace => {
        self.position += 1;
      },
      _ => panic!("Failed to parse function")
    }

    // skip matching function parameters for now

    match self.tokens[self.position]{
      lex::LexerToken::RightRoundBrace => {
        self.position += 1;
      },
      _ => panic!("Failed to parse function")
    }

    // skip matching return type
    
    // skip matching where clause

    let block_expression = self.parse_block_expression();    

    return Some(cst::CstFunction::Create(
        qualifier: qualifiers,
        identifier: ident_token.String,
        block_expression: block_expression
      );
    )

  fn parse_function_qualifiers(&mut self) -> cst::CstFunctionQualifier{
    // doesn't handle extern yet
    let is_const = match self.tokens[self.position]{
      lex::LexerToken::Const => {
        self.position += 1;
        true
      },
      _ => false
    };


    let is_async = match self.tokens[self.position]{
      lex::LexerToken::Async => {
        self.position += 1;
        true
      },
      _ => false
    };
  
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
