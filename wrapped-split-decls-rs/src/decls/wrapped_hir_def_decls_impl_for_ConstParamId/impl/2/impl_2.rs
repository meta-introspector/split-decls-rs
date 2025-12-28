use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ConstParamId { # [doc = " Caller should check if this toc id really belongs to a const"] pub fn from_unchecked (it : TypeOrConstParamId) -> Self { Self (it) } }
}