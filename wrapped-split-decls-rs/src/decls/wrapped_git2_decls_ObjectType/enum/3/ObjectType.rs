use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " An enumeration all possible kinds objects may have."] # [derive (PartialEq , Eq , Copy , Clone , Debug)] pub enum ObjectType { # [doc = " Any kind of git object"] Any , # [doc = " An object which corresponds to a git commit"] Commit , # [doc = " An object which corresponds to a git tree"] Tree , # [doc = " An object which corresponds to a git blob"] Blob , # [doc = " An object which corresponds to a git tag"] Tag , }
}