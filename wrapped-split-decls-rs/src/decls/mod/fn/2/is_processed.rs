use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [inline] fn is_processed (v : PreorderIndex , lastlinked : Option < PreorderIndex >) -> bool { if let Some (ll) = lastlinked { v >= ll } else { false } }