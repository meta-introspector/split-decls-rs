use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a > super :: ForestObligation for & 'a str { type CacheKey = & 'a str ; fn as_cache_key (& self) -> Self :: CacheKey { self } }