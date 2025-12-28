use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone)] struct NodeLocality { index : usize , name : String , local_deps : Vec < String > , global_deps : Vec < String > , local_ratio : f64 , complexity : usize , }
}