use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl TypeParamId { # [doc = " Caller should check if this toc id really belongs to a type"] pub fn from_unchecked (it : TypeOrConstParamId) -> Self { Self (it) } }