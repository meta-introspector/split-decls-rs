use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn sanitize_id (s : & str) -> String { s . chars () . map (| c | if c . is_alphanumeric () || c == '_' { c } else { '_' }) . collect () }