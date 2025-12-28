use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct Attributes { ignore : bool , project : Option < Ident > , }
}