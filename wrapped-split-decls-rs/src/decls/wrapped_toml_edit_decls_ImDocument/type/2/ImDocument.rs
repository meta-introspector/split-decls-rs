use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " Type representing a parsed TOML document"] # [deprecated (since = "0.23.0" , note = "Replaced with `Document`")] pub type ImDocument < S > = Document < S > ;
}