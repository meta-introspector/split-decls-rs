use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PlaceholderExpander { pub fn add (& mut self , id : ast :: NodeId , mut fragment : AstFragment) { panic ! ("AstFragment.mut_visit_with called - functionality temporarily disabled") ; self . expanded_fragments . insert (id , fragment) ; } fn remove (& mut self , id : ast :: NodeId) -> AstFragment { self . expanded_fragments . remove (& id) . unwrap () } }
}