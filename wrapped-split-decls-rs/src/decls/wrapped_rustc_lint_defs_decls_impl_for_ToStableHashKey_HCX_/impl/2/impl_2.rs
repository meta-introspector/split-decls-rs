use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < HCX > ToStableHashKey < HCX > for LintId { type KeyType = & 'static str ; # [inline] fn to_stable_hash_key (& self , _ : & HCX) -> & 'static str { self . lint_name_raw () } }