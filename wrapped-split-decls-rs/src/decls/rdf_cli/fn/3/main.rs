use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn main () -> Result < () > { let mut interpreter = RdfInterpreter :: new () ; println ! ("🚀 Loading LMDFB Knowledge Base...") ; interpreter . load_kb ("rustc_lmdfb_knowledge_base.owl") ? ; println ! ("✅ Knowledge base loaded successfully!") ; interpreter . show_stats () ; interpreter . show_predicates () ; interpreter . run_cli () ; Ok (()) }
}