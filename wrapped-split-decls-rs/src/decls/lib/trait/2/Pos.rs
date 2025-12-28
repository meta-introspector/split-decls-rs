use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait Pos { fn from_usize (n : usize) -> Self ; fn to_usize (& self) -> usize ; fn from_u32 (n : u32) -> Self ; fn to_u32 (& self) -> u32 ; }