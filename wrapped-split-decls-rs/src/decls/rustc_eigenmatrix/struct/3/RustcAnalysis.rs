use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] pub struct RustcAnalysis { # [doc = " All rustc crates and their relationships"] pub crates : HashMap < String , CrateFeatures > , # [doc = " Compiler phases (parsing, analysis, codegen, etc.)"] pub phases : Vec < CompilerPhase > , # [doc = " Total lines of code"] pub total_loc : usize , # [doc = " Complexity metrics"] pub complexity : ComplexityMetrics , }
}