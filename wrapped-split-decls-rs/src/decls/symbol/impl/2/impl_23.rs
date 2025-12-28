use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl MacroRulesNormalizedIdent { # [inline] pub fn new (ident : Ident) -> Self { MacroRulesNormalizedIdent (ident . normalize_to_macro_rules ()) } }
}