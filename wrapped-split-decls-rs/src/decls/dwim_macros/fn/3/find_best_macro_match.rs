use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn find_best_macro_match (intent : & DwimIntent , macros : & HashMap < String , MacroDefinition >) -> MacroMatch { let matches : Vec < _ > = macros . values () . filter (| m | semantic_match_score (intent , m) > 0.7) . collect () ; match matches . len () { 0 => MacroMatch :: None , 1 => MacroMatch :: Single (matches [0] . clone ()) , _ => MacroMatch :: Ambiguous (matches . into_iter () . cloned () . collect ()) , } }
}