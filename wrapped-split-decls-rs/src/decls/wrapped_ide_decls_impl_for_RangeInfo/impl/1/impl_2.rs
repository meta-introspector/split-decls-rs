use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > RangeInfo < T > { pub fn new (range : TextRange , info : T) -> RangeInfo < T > { RangeInfo { range , info } } }