use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait HasDeclaredIdents : Sized { fn declared_idents (& self) -> SmallVec < Ident , 1 > ; }
}