use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: main");
fn main () { println ! ("Testing includemod! macro") ; let result = test_mod :: test_function () ; println ! ("Result: {:?}" , result) ; }
}