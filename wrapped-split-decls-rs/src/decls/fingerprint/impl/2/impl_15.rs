use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl FromStableHash for Fingerprint { type Hash = StableHasherHash ; # [inline] fn from (StableHasherHash ([_0 , _1]) : Self :: Hash) -> Self { Fingerprint (_0 , _1) } }