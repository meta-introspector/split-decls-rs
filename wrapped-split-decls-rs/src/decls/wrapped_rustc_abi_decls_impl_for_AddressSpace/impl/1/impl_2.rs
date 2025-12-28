use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl AddressSpace { # [doc = " LLVM's `0` address space."] pub const ZERO : Self = AddressSpace (0) ; }
}