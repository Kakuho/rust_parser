mod cst;

struct Program{
  buffer: String
}

struct Lexer{
  position: usize
}

enum LexerTokens{
  Char(char),
  // keywords, could be stored in a  hashmap instead 
  Fn,
  Const,
  Async,
  Safe,
  Unsafe,
  Extern,
  Abi(String),
  LeftCurlyBrace,
  RightCurlyBrace
}

struct Parser{
  position: usize,
  tokens: Vec<LexerTokens>
}

impl Parser{
  fn Parse(self) -> i8{
    // move the lexer into this function call and kill it self
    2
  }

  fn ParseBlockExpression(&mut self) -> Option<cst::BlockExpression>{
    // for now just matches the braces
    match self.tokens[self.position]{
      LexerTokens::LeftCurlyBrace => self.position = self.position+1,
      _ => panic!("damn bro")
    };
    match self.tokens[self.position]{
      LexerTokens::RightCurlyBrace => self.position = self.position+1,
      _ => panic!("damn bro")
    };
    return Some(CstBlockExpression{});
  }
 
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

  let sampleTokens: Vec<LexerTokens> = vec![
    LexerTokens::LeftCurlyBrace,
    LexerTokens::RightCurlyBrace
  ];

  let mut parser = Parser{
    position: 0,
    tokens: sampleTokens
  };

  let blockCst = parser.ParseBlockExpression();
  match blockCst{
    None => {},
    Some(node) => node.Print()
  };
}
