use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A FileLoader that uses std::fs to load real files."] pub struct RealFileLoader ;
}