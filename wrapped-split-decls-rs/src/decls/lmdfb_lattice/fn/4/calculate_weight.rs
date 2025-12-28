use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: calculate_weight");
fn calculate_weight (symbol : & str) -> f64 { let mut weight = 1.0 ; if symbol . contains ("rustc_") { weight *= 3.0 ; } if symbol . contains ("codegen") { weight *= 2.5 ; } if symbol . contains ("parse") { weight *= 2.0 ; } if symbol . contains ("ast") { weight *= 2.0 ; } if symbol . contains ("alloc") { weight *= 1.8 ; } if symbol . contains ("hash") { weight *= 1.5 ; } if symbol . contains ("io") { weight *= 1.3 ; } weight / (symbol . len () as f64 / 20.0) . max (1.0) }
}