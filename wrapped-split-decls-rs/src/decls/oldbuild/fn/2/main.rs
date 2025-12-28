use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main () -> Result < () > { println ! ("cargo:rerun-if-changed=build.rs") ; println ! ("cargo:rerun-if-changed=.split-decls-config.toml") ; println ! ("cargo:note=build.rs finished. If split-decls-rs needs to be re-run, changes will be detected.") ; Ok (()) }