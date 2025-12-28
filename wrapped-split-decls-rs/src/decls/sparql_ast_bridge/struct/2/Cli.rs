use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Parser)] # [command (name = "sparql-ast-bridge")] # [command (about = "SPARQL to AST Probe Bridge - Convert RDF queries to dynamic AST transformations")] struct Cli { # [command (subcommand)] command : Commands , }