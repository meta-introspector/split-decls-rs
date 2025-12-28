use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " \"Discriminant\" of an AST fragment."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum AstFragmentKind { OptExpr , MethodReceiverExpr , Expr , Pat , Ty , Stmts , Items , TraitItems , ImplItems , TraitImplItems , ForeignItems , Arms , ExprFields , PatFields , GenericParams , Params , FieldDefs , Variants , WherePredicates , Crate , }
}