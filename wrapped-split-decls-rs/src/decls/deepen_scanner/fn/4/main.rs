use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn main () -> Result < () > { println ! ("🔍 Deepen Scanner - Self-Analysis Mode") ; let mut tape_macros = HashMap :: new () ; let self_code = fs :: read_to_string ("src/bin/deepen_scanner.rs") ? ; tape_macros . insert ("deepen_scanner_self" . to_string () , self_code) ; println ! ("📼 Loaded {} macros from tape (self-code)" , tape_macros . len ()) ; let output2_blocks = scan_output2_blocks () ? ; println ! ("📂 Found {} blocks in output2" , output2_blocks . len ()) ; let similarities = find_similarities (& tape_macros , & output2_blocks) ? ; print_similarity_report (& similarities) ; Ok (()) }
}