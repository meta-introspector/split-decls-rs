use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for SplitDeclsConfig { fn default () -> Self { Self { patches : HashMap :: new () , string_replacements : HashMap :: new () , custom_prelude_overlay : None , } } }
}