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

  let function_cst = parser.parse_function();
  match function_cst{
    None => {},
    Some(node) => {node.Print()}
  };
}

fn ParseBlockExpression(){
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
}

fn ParseStatements(){
  // statements we wana parse for now:
  //   let k = 10;
  //   let mut j = 10;
  //   let mut j: i8 = 10;
  //   let mut j: f16 = 10;
  let sample_tokens: Vec<lex::LexerToken> = vec![
    //   let k = 10;
    lex::LexerToken::Let,
    lex::LexerToken::Identifier(String::from("k")),
    lex::LexerToken::Equal,
    lex::LexerToken::Integral(10),
    lex::LexerToken::SemiColon,
    //   let mut j = 10;
  ];

  let mut parser = concrete::Parser::create(sample_tokens);

  let let_statement: Option<concrete::cst::CstLetStatement> = parser.parse_let_statement();
  match let_statement{ None => {},
    Some(node) => {node.Print()}
  };
}

fn main(){
  let program = Program{
    buffer: String::from("
      fn main(){}
    ")
  };
  ParseStatements();
  println!("~Mero mero");
}
