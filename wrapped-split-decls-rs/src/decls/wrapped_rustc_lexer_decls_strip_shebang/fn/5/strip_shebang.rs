use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: strip_shebang");
# [doc = " `rustc` allows files to have a shebang, e.g. \"#!/usr/bin/rustrun\","] # [doc = " but shebang isn't a part of rust syntax."] pub fn strip_shebang (input : & str) -> Option < usize > { if let Some (input_tail) = input . strip_prefix ("#!") { let next_non_whitespace_token = tokenize (input_tail , FrontmatterAllowed :: No) . map (| tok | tok . kind) . find (| tok | { ! matches ! (tok , TokenKind :: Whitespace | TokenKind :: LineComment { doc_style : None } | TokenKind :: BlockComment { doc_style : None , .. }) }) ; if next_non_whitespace_token != Some (TokenKind :: OpenBracket) { return Some (2 + input_tail . lines () . next () . unwrap_or_default () . len ()) ; } } None }
}