use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Solution to a diagnostic item."] # [derive (Debug , Clone , Hash , PartialEq , Eq)] pub struct Solution { # [doc = " The error message of the diagnostic item."] pub message : String , # [doc = " Possible solutions to fix the error."] pub replacements : Vec < Replacement > , }