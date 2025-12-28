use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct MacroRulesMacroExpander < DRT : OpaqueDeriveResolution + 'static > { pub node_id : ast :: NodeId , pub name : Ident , pub span : Span , pub transparency : Transparency , pub kinds : MacroKinds , pub rules : Vec < MacroRule > , }
}