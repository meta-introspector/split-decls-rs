use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type NamedMatches = FxHashMap < MacroRulesNormalizedIdent , NamedMatch > ;
}