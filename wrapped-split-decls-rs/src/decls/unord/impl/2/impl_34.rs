use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < K , Q : ? Sized , V > Index < & Q > for UnordMap < K , V > where K : Eq + Hash + Borrow < Q > , Q : Eq + Hash , { type Output = V ; # [inline] fn index (& self , key : & Q) -> & V { & self . inner [key] } }