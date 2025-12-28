use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main () { println ! ("cargo:rerun-if-changed=build.rs") ; println ! ("cargo:rerun-if-changed=build_src/") ; println ! ("cargo:rerun-if-changed=build_src/os/") ; example_module :: run () ; os_specific_logic :: run () ; use cargo_toml_generator_macros :: define_root_cargo_toml ; use cargo_toml_generator_types :: CargoToml ; extern crate cargo_toml_parts ; let _generated_cargo_toml : CargoToml = define_root_cargo_toml ! () ; }