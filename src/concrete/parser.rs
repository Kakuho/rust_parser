use std::{collections::HashSet, ops::Deref};

use crate::concrete::cst::{CstFunctionQualifier, PatternNoTopAltKind};
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

  pub fn parse_function(&mut self) -> Option<cst::CstFunction>{
    let qualifiers = self.parse_function_qualifiers();

    match self.tokens[self.position]{
      lex::LexerToken::Fn => {
        self.position += 1;
      },
      _ => panic!("Failed to parse function")
    }

    let ref ident_token = self.tokens[self.position];

    let identifier: String = match ident_token{
      lex::LexerToken::Identifier(ident) => {
        self.position += 1;
        String::clone(ident)
      },
      _ => panic!("Failed to parse function")
    };

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

    return Some(
      cst::CstFunction::Create(qualifiers, identifier, block_expression)
    )
  }

  fn parse_function_qualifiers(&mut self) -> Option<cst::CstFunctionQualifier>{
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

    return Some(
      cst::CstFunctionQualifier{
        is_const: is_const,
        is_async: is_async,
        is_safe: is_safe,
        is_unsafe: is_unsafe
      }
    )
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

  pub fn parse_let_statement(&mut self) -> Option<cst::CstLetStatement>{
    match self.tokens[self.position]{
      lex::LexerToken::Let => self.position = self.position+1,
      _ => panic!("damn bro")
    };

    let pattern = self.parse_pattern_no_top_alt();

    match &pattern{
      Some(pattern) => self.position = self.position+1,
      _ => panic!("damn bro")
    };

    // see if there's a type
    let saved_pos = self.position;
    let possible_type = self.try_parse_let_type();

    let type_val = if possible_type.is_none(){
      self.position = saved_pos; // backtracking
      None
    }
    else {
      possible_type
    };

    match self.tokens[self.position]{
      lex::LexerToken::Equal => self.position = self.position+1,
      _ => {return None;}
    };

    let expression = self.parse_expression();

    return Some(cst::CstLetStatement{pattern: pattern, let_type: type_val, expression: expression});
  }

  pub fn try_parse_let_type(&mut self) -> Option<String>{
    match self.tokens[self.position]{
      lex::LexerToken::Colon => self.position = self.position+1,
      _ => {return None;}
    };

    let type_val = match &self.tokens[self.position]{
      lex::LexerToken::Type(type_lexeme) => String::clone(type_lexeme),
      _ => {return None;}
    };

    return Some(type_val);
  }

  pub fn parse_expression(&mut self) -> Option<cst::CstExpression>{
    let lexp = self.parse_literal_expression();
    match lexp {
      None => {}
      Some(litexp) => {
        return Some(
          cst::CstExpression{
            kind: cst::ExpressionKind::LiteralExpression(litexp)
          }
        );
      }
    }
    return None;
  }

  pub fn parse_literal_expression(&mut self) -> Option<cst::CstLiteralExpression>{
    let saved_pos = self.position;

    let char_literal = self.try_parse_char_literal();
    match char_literal{
      None => {self.position = saved_pos;}
      Some(literal_exp) => {return Some(literal_exp);}
    };
    
    let string_literal = self.try_parse_string_literal();
    match string_literal{
      None => {self.position = saved_pos;}
      Some(literal_exp) => {return Some(literal_exp);}
    };

    let int_literal = self.try_parse_int_literal();
    match int_literal{
      None => {self.position = saved_pos;}
      Some(literal_exp) => {return Some(literal_exp);}
    };

    let float_literal = self.try_parse_float_literal();
    match float_literal{
      None => {self.position = saved_pos;}
      Some(literal_exp) => {return Some(literal_exp);}
    };
    
    return None;  
  }

  fn try_parse_char_literal(&mut self) -> Option<cst::CstLiteralExpression>{
    match self.tokens[self.position]{
      lex::LexerToken::SingleQuote => self.position += 1,
      _ => {return None;}
    };

    let char_val =  match self.tokens[self.position]{
      lex::LexerToken::Character(charval) => {
        self.position += 1;
        charval
      }
      _ => {return None;}
    };

    match self.tokens[self.position]{
      lex::LexerToken::SingleQuote => self.position = self.position+1,
      _ => {return None;}
    };
      
    return Some(cst::CstLiteralExpression::from(char_val));
  }

  fn try_parse_string_literal(&mut self) -> Option<cst::CstLiteralExpression>{
    match self.tokens[self.position]{
      lex::LexerToken::DoubleQuote => self.position += 1,
      _ => {return None;}
    };

    let string_val =  match &self.tokens[self.position]{
      lex::LexerToken::String(string) => {
        self.position += 1;
        String::clone(string)
      }
      _ => {return None;}
    };

    match self.tokens[self.position]{
      lex::LexerToken::DoubleQuote => self.position += 1,
      _ => {return None;}
    };
      
    return Some(cst::CstLiteralExpression::from(string_val));
  }

  fn try_parse_int_literal(&mut self) -> Option<cst::CstLiteralExpression>{
    // should we do parsing of 0x, 0b, 00 prefixes?
    match self.tokens[self.position]{
      lex::LexerToken::Integral(val) => {
        self.position += 1;
        return Some(cst::CstLiteralExpression::from(val));
      },
      _ => {return None;}
    };
  }

  fn try_parse_float_literal(&mut self) -> Option<cst::CstLiteralExpression>{
    match self.tokens[self.position]{
      lex::LexerToken::Integral(val) => {
        self.position += 1;
        return Some(cst::CstLiteralExpression::from(val));
      },
      _ => {return None;}
    };
  }

  pub fn parse_pattern_no_top_alt(&mut self) -> Option<cst::CstPatternNoTopAlt>{
    // for now it only expects a pattern_without_range
    let pattern = self.try_parse_pattern_without_range();
    if(pattern.is_none()){
      panic!("Failure in parse_pattern_no_top_alt_kind: the expected pattern_without_range is null");
    }
    return Some(cst::CstPatternNoTopAlt::from(pattern.unwrap()));
  }

  fn try_parse_pattern_without_range(&mut self) -> Option<cst::CstPatternWithoutRange>{
    let savedpos = self.position;
    let identifier_pattern = self.try_parse_identifier_pattern();
    if(identifier_pattern.is_none()){
      self.position = savedpos;
    }
    else{
      return Some(cst::CstPatternWithoutRange::from(identifier_pattern.unwrap()));
    }
    return None;
  }

  fn try_parse_identifier_pattern(&mut self) -> Option<cst::CstIdentifierPattern>{
    let is_ref = match self.tokens[self.position]{
      lex::LexerToken::Ref => { self.position += 1; true},
      _ => {false}
    };

    let is_mut = match self.tokens[self.position]{
      lex::LexerToken::Mut => { self.position += 1; true},
      _ => {false}
    };

    let identifier = match &self.tokens[self.position]{
      lex::LexerToken::Identifier(identifier) => { String::clone(&identifier)},
      _ => {return None;}
    };

    Some(cst::CstIdentifierPattern::create(is_ref, is_mut, identifier))
  }

}
