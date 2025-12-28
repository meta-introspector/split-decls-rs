use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct WrongFragmentKind < 'a > { pub span : Span , pub kind : & 'a str , pub name : & 'a rustc_ast :: Path , }
}