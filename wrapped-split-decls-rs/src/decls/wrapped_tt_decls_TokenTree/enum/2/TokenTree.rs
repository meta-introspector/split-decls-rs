use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum TokenTree < S = u32 > { Leaf (Leaf < S >) , Subtree (Subtree < S >) , }
}