use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main () -> Result < () > { let test_crate_path = PathBuf :: from ("/mnt/data1/nix/vendor/rust/cargo2nix/unimacro_derive") ; let mut config = crate :: config_macros :: GLOBAL_CONFIG . lock () . unwrap () . clone () ; config . wrapping = mkwrapping ! () ; println ! ("Running decl splitter on unimacro_derive crate: {}" , test_crate_path . display ()) ; process_crate (& test_crate_path , & config , false) ? ; println ! ("Decl splitting completed successfully!") ; Ok (()) }