use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Ranker < 'a > { pub kind : parser :: SyntaxKind , pub text : & 'a str , pub ident_kind : bool , }