use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct SheblingArgs { path_key : Ident , _colon1 : Token ! [:] , path : LitStr , }
}