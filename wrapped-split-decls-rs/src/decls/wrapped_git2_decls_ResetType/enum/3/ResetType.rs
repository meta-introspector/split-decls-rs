use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " An enumeration of the operations that can be performed for the `reset`"] # [doc = " method on a `Repository`."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum ResetType { # [doc = " Move the head to the given commit."] Soft , # [doc = " Soft plus reset the index to the commit."] Mixed , # [doc = " Mixed plus changes in the working tree are discarded."] Hard , }
}