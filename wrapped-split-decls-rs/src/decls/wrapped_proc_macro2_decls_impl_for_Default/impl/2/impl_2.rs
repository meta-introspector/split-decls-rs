use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [doc = " `TokenStream::default()` returns an empty stream,"] # [doc = " i.e. this is equivalent with `TokenStream::new()`."] impl Default for TokenStream { fn default () -> Self { TokenStream :: new () } }
}