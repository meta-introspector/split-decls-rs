use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct UseGroup2 { pub brace_token : syn :: token :: Brace , pub items : syn :: punctuated :: Punctuated < UseTree2 , syn :: Token ! [,] > , }
}