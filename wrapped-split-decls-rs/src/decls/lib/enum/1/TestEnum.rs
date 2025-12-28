use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
pub enum TestEnum { Variant1 , Variant2 (i32) , }
}