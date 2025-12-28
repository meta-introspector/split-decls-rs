use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: is_sorted");
fn is_sorted < T : Send + Ord > (v : & [T]) -> bool { (1 .. v . len ()) . all (| i | v [i - 1] <= v [i]) }
}