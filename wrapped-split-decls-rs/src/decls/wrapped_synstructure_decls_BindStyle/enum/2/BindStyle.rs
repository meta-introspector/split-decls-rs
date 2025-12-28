use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " The type of binding to use when generating a pattern."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] pub enum BindStyle { # [doc = " `x`"] Move , # [doc = " `mut x`"] MoveMut , # [doc = " `ref x`"] Ref , # [doc = " `ref mut x`"] RefMut , }
}