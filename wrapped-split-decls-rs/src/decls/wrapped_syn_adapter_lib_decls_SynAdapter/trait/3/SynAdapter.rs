use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A trait for parsing Rust code, abstracting different parsing implementations."] pub trait SynAdapter : Send + Sync { # [doc = " Parses a Rust file from a given path into a `syn::File` (or equivalent AST)."] fn parse_file (& self , path : & Path) -> Result < File > ; # [doc = " Parses a string containing Rust code into a `syn::File` (or equivalent AST)."] fn parse_str (& self , code : & str) -> Result < File > ; # [doc = " Returns a reference to `Any` for downcasting."] fn as_any (& self) -> & dyn Any ; }