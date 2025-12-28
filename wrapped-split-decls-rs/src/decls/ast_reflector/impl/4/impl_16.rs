use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl AstNodeType { fn as_str (& self) -> & 'static str { match self { AstNodeType :: Function => "Function" , AstNodeType :: Struct => "Struct" , AstNodeType :: Enum => "Enum" , AstNodeType :: Impl => "Impl" , AstNodeType :: Trait => "Trait" , AstNodeType :: Module => "Module" , AstNodeType :: Use => "Use" , AstNodeType :: Const => "Const" , AstNodeType :: Static => "Static" , AstNodeType :: Type => "Type" , AstNodeType :: Macro => "Macro" , AstNodeType :: Expr => "Expr" , AstNodeType :: Stmt => "Stmt" , AstNodeType :: Pat => "Pat" , AstNodeType :: All => "All" , } } }
}