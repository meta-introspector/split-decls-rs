use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl BoundPolarity { pub fn as_str (self) -> & 'static str { match self { Self :: Positive => "" , Self :: Negative (_) => "!" , Self :: Maybe (_) => "?" , } } }
}