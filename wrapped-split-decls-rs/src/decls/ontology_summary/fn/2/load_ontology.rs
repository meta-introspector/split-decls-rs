use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn load_ontology (filename : & str) -> Result < RustcOntology > { let content = fs :: read_to_string (filename) ? ; Ok (serde_json :: from_str (& content) ?) }
}