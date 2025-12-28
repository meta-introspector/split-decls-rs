use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: analyze_crate");
fn analyze_crate (crate_path : & Path , name : String) -> Result < CrateMetrics > { let mut metrics = CrateMetrics { name , .. Default :: default () } ; let src_path = crate_path . join ("src") ; if ! src_path . exists () { return Ok (metrics) ; } analyze_rust_files (& src_path , & mut metrics) ? ; Ok (metrics) }
}