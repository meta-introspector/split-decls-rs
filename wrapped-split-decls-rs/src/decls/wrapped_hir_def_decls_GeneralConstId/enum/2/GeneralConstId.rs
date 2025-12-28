use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A constant, which might appears as a const item, an anonymous const block in expressions"] # [doc = " or patterns, or as a constant in types with const generics."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , salsa_macros :: Supertype)] pub enum GeneralConstId { ConstId (ConstId) , StaticId (StaticId) , }