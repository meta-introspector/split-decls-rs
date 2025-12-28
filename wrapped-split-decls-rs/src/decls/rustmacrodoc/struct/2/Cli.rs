use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Parser , Debug)] # [command (author , version , about , long_about = None)] struct Cli { # [command (subcommand)] command : Commands , }
}