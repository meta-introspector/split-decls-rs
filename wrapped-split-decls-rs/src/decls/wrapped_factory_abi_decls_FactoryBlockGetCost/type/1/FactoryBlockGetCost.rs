use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type FactoryBlockGetCost = extern "C" fn (block_ptr : * mut c_void) -> u32 ;
}