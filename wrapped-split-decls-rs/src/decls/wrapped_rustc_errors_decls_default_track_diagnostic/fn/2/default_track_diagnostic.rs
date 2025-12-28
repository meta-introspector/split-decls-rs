use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: default_track_diagnostic");
fn default_track_diagnostic < R > (diag : DiagInner , f : & mut dyn FnMut (DiagInner) -> R) -> R { (* f) (diag) }
}