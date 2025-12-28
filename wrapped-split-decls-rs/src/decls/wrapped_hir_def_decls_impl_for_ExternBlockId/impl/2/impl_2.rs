use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [salsa :: tracked] impl ExternBlockId { # [salsa :: tracked] pub fn abi (self , db : & dyn DefDatabase) -> Option < Symbol > { signatures :: extern_block_abi (db , self) } }