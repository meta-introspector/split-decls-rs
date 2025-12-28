use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Read a line of text from stdin."] pub fn read_line () -> Result < String , io :: Error > { let mut buf = String :: new () ; io :: stdin () . read_line (& mut buf) ? ; Ok (buf . trim () . to_string ()) }