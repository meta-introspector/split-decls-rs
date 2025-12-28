use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl BoundConstness { pub fn as_str (self) -> & 'static str { match self { Self :: Never => "" , Self :: Always (_) => "const" , Self :: Maybe (_) => "[const]" , } } }