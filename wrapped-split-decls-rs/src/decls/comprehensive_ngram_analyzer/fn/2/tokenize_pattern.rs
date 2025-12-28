use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn tokenize_pattern (pattern : & str) -> Vec < String > { pattern . split (['/' , '_' , '.' , '-' , ':' , ' ']) . filter (| s | ! s . is_empty () && s . len () > 0) . map (| s | s . to_lowercase ()) . collect () }