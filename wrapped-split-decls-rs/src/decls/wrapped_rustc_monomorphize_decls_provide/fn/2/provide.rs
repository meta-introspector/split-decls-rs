use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: provide");
pub fn provide (providers : & mut Providers) { partitioning :: provide (providers) ; mono_checks :: provide (providers) ; }
}