use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < DRT : OpaqueDeriveResolution + 'static > MacroKindTrait for MacroRulesMacroExpander < DRT > { fn get_name (& self) -> String { "macro_rules" . to_string () } }