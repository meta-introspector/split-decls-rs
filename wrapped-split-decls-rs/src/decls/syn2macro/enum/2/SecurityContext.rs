use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Security and ACL enum for AST operations  "] # [derive (Debug , Clone)] pub enum SecurityContext { Default , Strict (Vec < AstOperation >) , }
}