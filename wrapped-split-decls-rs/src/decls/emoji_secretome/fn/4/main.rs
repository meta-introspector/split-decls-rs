use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn main () -> Result < () > { let output_dir = Path :: new ("output3") ; let mut secretome = HashMap :: new () ; for entry in fs :: read_dir (output_dir) ? { let entry = entry ? ; if entry . file_name () . to_string_lossy () . starts_with ("wrapped-") { process_crate (& entry . path () , & mut secretome) ? ; } } let emoji_map = generate_emoji_mappings (& secretome) ; fs :: write ("rustc_secretome.json" , serde_json :: to_string_pretty (& emoji_map) ?) ? ; println ! ("🧬 Generated rustc secretome with {} unique symbols" , emoji_map . len ()) ; Ok (()) }
}