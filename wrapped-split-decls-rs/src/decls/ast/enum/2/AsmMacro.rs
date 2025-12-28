use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Copy , Encodable , Decodable , Debug , HashStable_Generic , Walkable , PartialEq , Eq)] pub enum AsmMacro { # [doc = " The `asm!` macro"] Asm , # [doc = " The `global_asm!` macro"] GlobalAsm , # [doc = " The `naked_asm!` macro"] NakedAsm , }
}