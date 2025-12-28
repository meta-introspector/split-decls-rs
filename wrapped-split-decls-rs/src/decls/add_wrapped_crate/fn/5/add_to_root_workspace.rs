use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn add_to_root_workspace (wrapped_name : & str) -> Result < () > { use std :: fs ; use toml_edit :: { DocumentMut , value } ; let root_cargo_path = "../../Cargo.toml" ; let content = fs :: read_to_string (root_cargo_path) ? ; let mut doc = content . parse :: < DocumentMut > () ? ; let members_array = doc . get_mut ("workspace") . and_then (| w | w . get_mut ("members")) . and_then (| m | m . as_array_mut ()) . ok_or_else (| | anyhow :: anyhow ! ("No workspace.members found")) ? ; let member_path = format ! ("submodules/split-decls-rs/output2/{}" , wrapped_name) ; let exists = members_array . iter () . any (| item | { item . as_str () == Some (& member_path) }) ; if ! exists { members_array . push (& member_path) ; println ! ("Added {} to workspace members" , member_path) ; fs :: write (root_cargo_path , doc . to_string ()) ? ; } else { println ! ("Member {} already exists" , member_path) ; } Ok (()) }
}