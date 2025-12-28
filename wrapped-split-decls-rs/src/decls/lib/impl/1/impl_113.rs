use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl SourceFileLines { pub fn is_lines (& self) -> bool { matches ! (self , SourceFileLines :: Lines (_)) } }