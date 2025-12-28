use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Token { fn new (kind : TokenKind , len : u32) -> Token { Token { kind , len } } }