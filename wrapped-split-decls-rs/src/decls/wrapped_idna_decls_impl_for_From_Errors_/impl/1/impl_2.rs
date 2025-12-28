use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Errors > for Result < () , Errors > { fn from (e : Errors) -> Self { Err (e) } }
}