mod concrete;
mod lex;

struct Program{
  buffer: String
}

// for now generate a concrete syntax tree of a simple program:
//   fn main(){
//   }
// 
// which should give the concrete syntax tree:
//   Crate
//     Item
//       VisItem
//         Function
//           Function Qualififer - null
//           Identifier
//             NonKeyWordIdentifier
//               IdentifierOrKeyword
//                 XID_Start - unicode??
//           GenericParams - null
//           FunctionParameters - null
//           FunctionReturnType - null
//           WhereClause - null
//           BlockExpression
//             InnerAttribute - null
//             Statements - null
//

fn ParseMainFunction(){
  let main_func_tokens: Vec<lex::LexerToken> = vec![
    lex::LexerToken::Fn,
    lex::LexerToken::Identifier(String::from("main")),
    lex::LexerToken::LeftRoundBrace,
    lex::LexerToken::RightRoundBrace,
    lex::LexerToken::LeftCurlyBrace,
    lex::LexerToken::RightCurlyBrace
  ];

  let mut parser = concrete::Parser::create(main_func_tokens);

  let function_cst: Option<concrete::cst::CstBlockExpression> = parser.parse_block_expression();
  match function_cst{
    None => {},
    Some(node) => {node.Print()}
  };
}

fn main(){
  let program = Program{
    buffer: String::from("
      fn main(){}
    ")
  };

  let sample_tokens: Vec<lex::LexerToken> = vec![
    lex::LexerToken::LeftCurlyBrace,
    lex::LexerToken::RightCurlyBrace
  ];

  let mut parser = concrete::Parser::create(sample_tokens);

  let blockCst: Option<concrete::cst::CstBlockExpression> = parser.parse_block_expression();
  match blockCst{
    None => {},
    Some(node) => {node.Print()}
  };
  
  println!("~Mero mero");
}
