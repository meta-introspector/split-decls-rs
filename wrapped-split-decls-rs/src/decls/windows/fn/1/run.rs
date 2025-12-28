use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn run () { println ! ("cargo:warning=Running build logic for Windows!") ; }
}