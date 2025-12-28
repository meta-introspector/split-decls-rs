use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl SuggestionStyle { fn hide_inline (& self) -> bool { ! matches ! (* self , SuggestionStyle :: ShowCode) } }
}