use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait ForestObligation : Clone + Debug { type CacheKey : Clone + hash :: Hash + Eq + Debug ; # [doc = " Converts this `ForestObligation` suitable for use as a cache key."] # [doc = " If two distinct `ForestObligations`s return the same cache key,"] # [doc = " then it must be sound to use the result of processing one obligation"] # [doc = " (e.g. success for error) for the other obligation"] fn as_cache_key (& self) -> Self :: CacheKey ; }