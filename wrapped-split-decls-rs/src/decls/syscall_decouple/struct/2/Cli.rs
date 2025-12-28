use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Parser)] # [command (name = "syscall-decouple")] # [command (about = "Generate trait-based syscall decoupling from AST and SPARQL analysis")] struct Cli { # [command (subcommand)] command : Commands , }