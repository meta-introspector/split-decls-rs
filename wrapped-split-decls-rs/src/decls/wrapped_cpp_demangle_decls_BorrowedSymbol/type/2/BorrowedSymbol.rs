use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A `Symbol` which borrows the underlying storage for the mangled name."] pub type BorrowedSymbol < 'a > = Symbol < & 'a [u8] > ;