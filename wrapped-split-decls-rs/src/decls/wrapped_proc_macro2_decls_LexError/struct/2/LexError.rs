use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Error returned from `TokenStream::from_str`."] pub struct LexError { inner : imp :: LexError , _marker : ProcMacroAutoTraits , }
}