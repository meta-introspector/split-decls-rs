use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn process_crate (crate_path : & Path , secretome : & mut HashMap < String , SymbolInfo >) -> Result < () > { let decls_dir = crate_path . join ("src/decls") ; if ! decls_dir . exists () { return Ok (()) ; } for entry in fs :: read_dir (decls_dir) ? { let entry = entry ? ; if entry . path () . extension () . map_or (false , | ext | ext == "rs") { let content = fs :: read_to_string (entry . path ()) ? ; if let Ok (file) = syn :: parse_file (& content) { extract_symbols (& file , secretome) ; } } } Ok (()) }