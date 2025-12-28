use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Whether to stop or continue compilation."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum Compilation { Stop , Continue , }