use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn generate_wrapped_cargo_toml (binary_name : & str) -> Result < String > { let wrapped_crate_name = format ! ("wrapped_{}" , binary_name . replace ("-" , "_")) ; Ok (format ! (r#"[package]
name = "{}-wrapped"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "{}"
path = "src/main.rs"

[dependencies]
{} = {{ path = "./{}" }}
anyhow = "1.0"
"# , binary_name , binary_name , wrapped_crate_name , wrapped_crate_name)) }