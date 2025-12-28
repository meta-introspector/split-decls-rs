use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: check");
fn check < D : Default + PartialEq > (result : D) -> D { if result == D :: default () { panic ! ("allocation failed") ; } result }
}