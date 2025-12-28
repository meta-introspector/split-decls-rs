use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl FromStableHash for Hash128 { type Hash = StableHasherHash ; # [inline] fn from (StableHasherHash ([_0 , _1]) : Self :: Hash) -> Self { Self { inner : u128 :: from (_0) | (u128 :: from (_1) << 64) } } }