use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Used to translate between `Span`s and byte positions within a single output line in highlighted"] # [doc = " code of structured suggestions."] # [derive (Debug , Clone , Copy)] pub struct SubstitutionHighlight { start : usize , end : usize , }