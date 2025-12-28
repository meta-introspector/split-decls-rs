use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a > IntoIterator for & 'a Utf8Path { type Item = & 'a str ; type IntoIter = Iter < 'a > ; # [inline] fn into_iter (self) -> Iter < 'a > { self . iter () } }