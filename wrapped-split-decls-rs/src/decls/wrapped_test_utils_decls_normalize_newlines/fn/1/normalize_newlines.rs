use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn normalize_newlines (s : & str) -> String { s . replace ("\r\n" , "\n") }