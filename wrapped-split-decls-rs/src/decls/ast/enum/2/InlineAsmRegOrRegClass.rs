use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Inline assembly operand explicit register or register class."] # [doc = ""] # [doc = " E.g., `\"eax\"` as in `asm!(\"mov eax, 2\", out(\"eax\") result)`."] # [derive (Clone , Copy , Encodable , Decodable , Debug , Walkable)] pub enum InlineAsmRegOrRegClass { Reg (Symbol) , RegClass (Symbol) , }
}