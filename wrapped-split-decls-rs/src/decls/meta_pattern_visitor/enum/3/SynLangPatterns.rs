use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum SynLangPatterns { File , Item , ItemFn , ItemStruct , ItemEnum , ItemImpl , ItemTrait , ItemMod , ItemUse , ItemConst , ItemStatic , Expr , ExprCall , ExprMethodCall , ExprPath , ExprLit , ExprBlock , ExprIf , ExprMatch , ExprBinary , ExprUnary , Type , TypePath , TypeReference , TypeTuple , Pat , PatIdent , PatStruct , PatTuple , Ident , Path , Block , Signature , Generics , }
}