use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Cow < 'static , str > > for DiagMessage { fn from (s : Cow < 'static , str >) -> Self { DiagMessage :: Str (s) } }
}