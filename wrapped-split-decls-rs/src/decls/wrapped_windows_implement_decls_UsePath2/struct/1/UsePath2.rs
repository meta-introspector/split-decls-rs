use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct UsePath2 { pub ident : syn :: Ident , pub tree : Box < UseTree2 > , }
}