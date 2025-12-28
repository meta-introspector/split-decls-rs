use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl FromStableHash for Hash64 { type Hash = StableHasherHash ; # [inline] fn from (StableHasherHash ([_0 , __1]) : Self :: Hash) -> Self { Self { inner : _0 } } }