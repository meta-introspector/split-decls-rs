use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Possible output formats for diff data"] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum DiffFormat { # [doc = " full git diff"] Patch , # [doc = " just the headers of the patch"] PatchHeader , # [doc = " like git diff --raw"] Raw , # [doc = " like git diff --name-only"] NameOnly , # [doc = " like git diff --name-status"] NameStatus , # [doc = " git diff as used by git patch-id"] PatchId , }
}