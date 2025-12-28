use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Generates the new Cargo.toml for the crate using lib-cargo processor"] pub fn generate_new_cargotoml (original_cargo_toml_path : & Path , output_cargo_toml_path : & Path , original_crate_root_path : & Path , global_config : & SplitDeclsConfig , patch_config : & patch_config :: PatchConfig , dry_run : bool ,) -> Result < () > { let processor = CargoProcessor :: new (global_config . clone ()) ; processor . process_cargo_toml (original_cargo_toml_path , output_cargo_toml_path , original_crate_root_path , dry_run ,) ? ; Ok (()) }
}