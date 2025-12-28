use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] pub struct SourceFileAndLine { pub sf : Arc < SourceFile > , # [doc = " Index of line, starting from 0."] pub line : usize , }