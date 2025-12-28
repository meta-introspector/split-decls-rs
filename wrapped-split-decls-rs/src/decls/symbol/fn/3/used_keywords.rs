use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Collect all the keywords in a given edition into a vector."] # [doc = ""] # [doc = " *Note:* Please update this if a new keyword is added beyond the current"] # [doc = " range."] pub fn used_keywords (edition : impl Copy + FnOnce () -> Edition) -> Vec < Symbol > { (kw :: DollarCrate . as_u32 () .. kw :: Yeet . as_u32 ()) . filter_map (| kw | { let kw = Symbol :: new (kw) ; if kw . is_used_keyword_always () || kw . is_used_keyword_conditional (edition) { Some (kw) } else { None } }) . collect () }