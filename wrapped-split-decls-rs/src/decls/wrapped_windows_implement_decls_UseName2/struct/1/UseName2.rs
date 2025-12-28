use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct UseName2 { pub ident : syn :: Ident , pub generics : Vec < UseTree2 > , }