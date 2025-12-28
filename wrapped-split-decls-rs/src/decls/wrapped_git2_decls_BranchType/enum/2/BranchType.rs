use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " An enumeration for the possible types of branches"] # [derive (PartialEq , Eq , Debug , Copy , Clone)] pub enum BranchType { # [doc = " A local branch not on a remote."] Local , # [doc = " A branch for a remote."] Remote , }
}