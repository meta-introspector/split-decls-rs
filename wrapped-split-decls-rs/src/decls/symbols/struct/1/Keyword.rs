use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct Keyword { name : Ident , value : LitStr , }
}