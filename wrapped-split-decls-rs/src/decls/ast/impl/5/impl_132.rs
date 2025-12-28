use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl YieldKind { # [doc = " Returns the expression inside the yield expression, if any."] # [doc = ""] # [doc = " For postfix yields, this is guaranteed to be `Some`."] pub const fn expr (& self) -> Option < & Box < Expr > > { match self { YieldKind :: Prefix (expr) => expr . as_ref () , YieldKind :: Postfix (expr) => Some (expr) , } } # [doc = " Returns a mutable reference to the expression being yielded, if any."] pub const fn expr_mut (& mut self) -> Option < & mut Box < Expr > > { match self { YieldKind :: Prefix (expr) => expr . as_mut () , YieldKind :: Postfix (expr) => Some (expr) , } } # [doc = " Returns true if both yields are prefix or both are postfix."] pub const fn same_kind (& self , other : & Self) -> bool { match (self , other) { (YieldKind :: Prefix (_) , YieldKind :: Prefix (_)) => true , (YieldKind :: Postfix (_) , YieldKind :: Postfix (_)) => true , _ => false , } } }
}