use super::expressions;

pub struct CstCrate{
  item_node: CstItem
}

pub struct CstItem{

}

enum VisItemChild{
  Module,
  // ...
  Function(CstFunction)
  // ...
}

pub struct CstVisItem{
  //visibility: Option<CstVisibility>
}

pub struct CstFunction{
  qualifier: Option<CstFunctionQualifier>,
  identifier: String, // string for now, later we improve on it and add the XID_Start and XID_Continue, w/e that means...
  //generic_params: Option<CstGenericParams> 
  //function_params: Option<CstFunctionParams> 
  //function_return_type: Option<CstFunctionReturnType> 
  //where_clause: Option<CstWhereClause> 
  block_expression: Option<expressions::CstBlockExpression>
}

pub struct CstFunctionQualifier{
  pub is_const: bool,
  pub is_async: bool,
  pub is_safe: bool,
  pub is_unsafe: bool
}

impl CstFunction{
  pub fn Create(qualifier: Option<CstFunctionQualifier>, identifier: String, block_expression: Option<expressions::CstBlockExpression>) -> CstFunction{
    CstFunction{
      qualifier: qualifier,
      identifier: identifier,
      block_expression: block_expression
    }
  }

  pub fn Print(&self){
    println!("Function");

    println!("Qualifiers:");
    match &self.qualifier{
      Some(qualifiers) => {
        if(qualifiers.is_const){
          println!("Const");
        }

        if(qualifiers.is_async){
          println!("Async");
        }

        if(qualifiers.is_unsafe){
          println!("Unsafe");
        }
        else{
          println!("Safe");
        }
      }
      None => {}
    }
    
    println!("Identifier: {}", self.identifier);

    match &self.block_expression{
      Some(blocknode) => {
        blocknode.Print();
      }
      None => {}
    }
  }
}
