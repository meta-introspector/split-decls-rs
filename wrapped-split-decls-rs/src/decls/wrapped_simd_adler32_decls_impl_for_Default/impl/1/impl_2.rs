use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for Adler32 { fn default () -> Self { Self { a : 1 , b : 0 , update : get_imp () , } } }
}