use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: calculate_complexity");
fn calculate_complexity (symbol : & str) -> f64 { let mut score = 1.0 ; score += symbol . len () as f64 / 10.0 ; score += symbol . matches ("::") . count () as f64 * 0.5 ; if symbol . contains ("impl") { score += 2.0 ; } if symbol . contains ("generic") { score += 1.5 ; } if symbol . contains ("macro") { score += 1.0 ; } score }
}