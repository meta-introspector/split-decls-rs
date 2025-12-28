use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: extract_dependencies");
fn extract_dependencies (symbol : & str) -> Vec < String > { let mut deps = Vec :: new () ; if symbol . contains ("::") { let parts : Vec < & str > = symbol . split ("::") . collect () ; if parts . len () > 1 { deps . push (parts [0] . to_string ()) ; } } deps }
}