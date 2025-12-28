use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn extract_crate_name (symbol : & str) -> String { symbol . split ("::") . next () . unwrap_or ("unknown") . to_string () }