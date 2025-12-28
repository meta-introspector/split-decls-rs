use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl RangeLimits { pub fn as_str (& self) -> & 'static str { match self { RangeLimits :: HalfOpen => ".." , RangeLimits :: Closed => "..=" , } } }