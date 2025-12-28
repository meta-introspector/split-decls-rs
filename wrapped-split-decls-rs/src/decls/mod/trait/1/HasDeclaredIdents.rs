use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait HasDeclaredIdents : Sized { fn declared_idents (& self) -> SmallVec < Ident , 1 > ; }