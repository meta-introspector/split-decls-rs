use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: foo");
pub fn foo () { println ! ("x") ; let mut map = HashMap :: new () ; map . insert (1 , "foo") ; }
}