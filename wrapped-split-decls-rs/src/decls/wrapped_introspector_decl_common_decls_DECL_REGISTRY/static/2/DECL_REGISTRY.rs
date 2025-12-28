use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
pub static DECL_REGISTRY : Lazy < Mutex < DeclRegistry > > = Lazy :: new (| | Mutex :: new (DeclRegistry :: default ())) ;
}