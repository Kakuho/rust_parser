// Supporting Code for implementing the concrete syntax tree

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
  block_expression: CstBlockExpression
}

pub struct CstFunctionQualifier{
  pub is_const: bool,
  pub is_async: bool,
  pub is_safe: bool,
  pub is_unsafe: bool
}

pub struct CstBlockExpression{
  // not parsing attributes for now, later will add attribute parsing
  // statements: Option<CstStatements>
}

impl CstBlockExpression{
  pub fn Print(&self){
    println!("Block Expression");
  }
}

