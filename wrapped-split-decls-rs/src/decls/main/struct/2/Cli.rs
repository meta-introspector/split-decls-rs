use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Parser , Debug)] # [command (author , version , about , long_about = None)] struct Cli { # [doc = " Turn on verbose output"] # [arg (short , long)] verbose : bool , # [command (subcommand)] command : Commands , }