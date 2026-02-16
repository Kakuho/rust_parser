mod concrete;
mod lexer;

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

fn main(){
  let program = Program{
    buffer: String::from("
      fn main(){}
    ")
  };

  let sampleTokens: Vec<lexer::LexerToken> = vec![
    lexer::LexerToken::LeftCurlyBrace,
    lexer::LexerToken::RightCurlyBrace
  ];

  let mut parser = concrete::Parser::create(sampleTokens);

  let blockCst: Option<concrete::cst::CstBlockExpression> = parser.parse_block_expression();
  match blockCst{
    None => {},
    Some(node) =>{
      let block: concrete::cst::CstBlockExpression = node;
      block.Print();
    }
  };
  println!("~Mero mero");
}
