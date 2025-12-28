use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " hir::Crate describes a single crate. It's the main interface with which"] # [doc = " a crate's dependencies interact. Mostly, it should be just a proxy for the"] # [doc = " root module."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Crate { pub (crate) id : base_db :: Crate , }