use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Parser)] # [command (name = "ast-reflector")] # [command (about = "AST Reflector - eBPF-like probes for Rust AST transformation")] struct Cli { # [command (subcommand)] command : Commands , }