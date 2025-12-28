use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: duration_to_secs_str");
pub fn duration_to_secs_str (dur : std :: time :: Duration) -> String { format ! ("{:.3}" , dur . as_secs_f64 ()) }
}