use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct Input { keywords : Punctuated < Keyword , Token ! [,] > , symbols : Punctuated < Symbol , Token ! [,] > , }
}