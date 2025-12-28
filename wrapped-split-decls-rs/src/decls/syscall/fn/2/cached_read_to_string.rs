use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn cached_read_to_string < P : AsRef < Path > > (path : P) -> anyhow :: Result < String > { let mut cache = GLOBAL_CACHE . lock () . unwrap () ; let content = cache . get_cached_content (path . as_ref ()) ? ; let _ = cache . save () ; Ok (content) }