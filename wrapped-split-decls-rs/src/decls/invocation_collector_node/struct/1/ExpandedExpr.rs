use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct ExpandedExpr (pub Box < ast :: Expr >) ;
}