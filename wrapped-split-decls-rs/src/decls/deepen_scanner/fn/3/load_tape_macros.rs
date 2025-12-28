use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn load_tape_macros () -> Result < HashMap < String , String > > { let mut macros = HashMap :: new () ; if Path :: new ("repl_state.rdf") . exists () { let rdf_content = fs :: read_to_string ("repl_state.rdf") ? ; for line in rdf_content . lines () { if line . contains ("sys:macro") { if let Some (name) = extract_macro_name (line) { if let Some (content) = extract_macro_content (line) { macros . insert (name , content) ; } } } } } Ok (macros) }
}