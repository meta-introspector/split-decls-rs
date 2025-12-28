use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl InlineAsmTemplatePiece { # [doc = " Rebuilds the asm template string from its pieces."] pub fn to_string (s : & [Self]) -> String { use fmt :: Write ; let mut out = String :: new () ; for p in s . iter () { let _ = write ! (out , "{p}") ; } out } }