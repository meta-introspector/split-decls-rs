use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main () -> Result < () > { let parent_cargo_path = Path :: new ("../../Cargo.toml") ; let current_dir = Path :: new (".") ; println ! ("Syncing workspace members to parent Cargo.toml...") ; let local_crates = find_local_workspace_crates (current_dir) ? ; update_parent_workspace (& parent_cargo_path , & local_crates) ? ; println ! ("Successfully synced {} workspace members" , local_crates . len ()) ; Ok (()) }