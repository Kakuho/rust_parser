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

///////////////////////////////////////////////////////////////////////////////////////

struct Parser{
  position: usize,
  tokens: Vec<LexerTokens>
}

struct CstCrate{
  item_node: CstItem
}

struct CstItem{
  
}

enum VisItemChild{
  Module,
  // ...
  Function(CstFunction)
  // ...
}

struct CstVisItem{
  //visibility: Option<CstVisibility>
  
}

struct CstFunction{
  qualifier: Option<CstFunctionQualifier>,
  identifier: String, // string for now, later we improve on it and add the XID_Start and XID_Continue, w/e that means...
  //generic_params: Option<CstGenericParams> 
  //function_params: Option<CstFunctionParams> 
  //function_return_type: Option<CstFunctionReturnType> 
  //where_clause: Option<CstWhereClause> 
  block_expression: CstBlockExpression
}

struct CstFunctionQualifier{
  is_const: bool,
  is_async: bool,
  is_safe: bool,
  is_unsafe: bool
}

struct CstBlockExpression{
  // not parsing attributes for now, later will add attribute parsing
  //statements: Option<CstStatements>
}

impl CstBlockExpression{

}

impl Parser{
  fn Parse(self) -> i8{
    // move the lexer into this function call and kill it self
    2
  }

  fn ParseBlockExpression(&mut self) -> Option<CstBlockExpression>{
    // for now just matches the braces
    match self.tokens[self.position]{
      LexerTokens::LeftCurlyBrace => self.position = self.position+1
      _ => panic!("damn bro")
    };
    match self.tokens[self.position]{
      LexerTokens::LeftCurlyBrace => self.position = self.position+1
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

fn main() {
  let program = Program{
    buffer: String::from("
      fn main(){}
    ")
  };

  let sampleTokens: Vec<LexerTokens> = vec![
    LexerTokens::LeftCurlyBrace,
    LexerTokens::RightCurlyBrace
  ];
}
      
