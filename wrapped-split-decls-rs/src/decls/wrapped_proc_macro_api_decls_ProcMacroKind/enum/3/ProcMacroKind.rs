use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Represents different kinds of procedural macros that can be expanded by the external server."] # [derive (Copy , Clone , Eq , PartialEq , Debug , serde_derive :: Serialize , serde_derive :: Deserialize)] pub enum ProcMacroKind { # [doc = " A macro that derives implementations for a struct or enum."] CustomDerive , # [doc = " An attribute-like procedural macro."] Attr , # [serde (alias = "Bang")] # [serde (rename (serialize = "FuncLike" , deserialize = "FuncLike"))] Bang , }