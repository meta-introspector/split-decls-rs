use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " A type alias for `TrieSetSlice<'static>`."] pub type TrieSet = TrieSetSlice < 'static > ;
}