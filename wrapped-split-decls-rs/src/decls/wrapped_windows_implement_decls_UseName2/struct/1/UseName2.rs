use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct UseName2 { pub ident : syn :: Ident , pub generics : Vec < UseTree2 > , }
}