use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type NamedMatches = FxHashMap < MacroRulesNormalizedIdent , NamedMatch > ;