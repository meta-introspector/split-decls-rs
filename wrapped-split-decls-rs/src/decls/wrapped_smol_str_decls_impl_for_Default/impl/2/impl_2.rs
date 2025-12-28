use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Default for SmolStrBuilderRepr { # [inline] fn default () -> Self { SmolStrBuilderRepr :: Inline { buf : [0 ; INLINE_CAP] , len : 0 , } } }
}