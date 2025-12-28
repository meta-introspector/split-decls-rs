use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ClosureKind { # [doc = " This is the initial value used when doing upvar inference."] pub const LATTICE_BOTTOM : ClosureKind = ClosureKind :: Fn ; pub const fn as_str (self) -> & 'static str { match self { ClosureKind :: Fn => "Fn" , ClosureKind :: FnMut => "FnMut" , ClosureKind :: FnOnce => "FnOnce" , } } # [doc = " Returns `true` if a type that impls this closure kind"] # [doc = " must also implement `other`."] # [rustfmt :: skip] pub fn extends (self , other : ClosureKind) -> bool { use ClosureKind :: * ; match (self , other) { (Fn , Fn | FnMut | FnOnce) | (FnMut , FnMut | FnOnce) | (FnOnce , FnOnce) => { true } _ => false , } } }
}