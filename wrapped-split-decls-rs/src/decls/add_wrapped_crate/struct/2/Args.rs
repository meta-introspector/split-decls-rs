use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Parser)] # [command (name = "add_wrapped_crate")] # [command (about = "Generate a wrapped crate and add it to root workspace")] struct Args { # [doc = " Path to the crate to wrap"] crate_path : String , # [doc = " Verbose output"] # [arg (short , long)] verbose : bool , }