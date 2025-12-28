use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < String > for DiagMessage { fn from (s : String) -> Self { DiagMessage :: Str (Cow :: Owned (s)) } }
}