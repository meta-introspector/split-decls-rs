use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A single change in a `Diff` that represents a range of tokens (`before`)"] # [doc = " in the first sequence that were replaced by a different range of tokens"] # [doc = " in the second sequence (`after`)."] # [doc = ""] # [doc = " Tokens that are a"] # [derive (Debug , Clone , PartialEq , Eq , Hash , Default)] pub struct Hunk { pub before : Range < u32 > , pub after : Range < u32 > , }
}