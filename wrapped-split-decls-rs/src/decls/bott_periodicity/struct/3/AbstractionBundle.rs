use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An abstraction at level n in the periodic tower"] # [derive (Debug , Clone)] pub struct AbstractionBundle { # [doc = " Which level (mod 8) in the Bott tower"] pub bott_level : BottLevel , # [doc = " Absolute level (how many times we've gone around)"] pub winding_number : usize , # [doc = " The actual content (shape repeats, but \"meaning\" differs)"] pub content : AbstractionContent , # [doc = " Characteristic classes (topological invariants)"] pub chern_classes : Vec < i32 > , }