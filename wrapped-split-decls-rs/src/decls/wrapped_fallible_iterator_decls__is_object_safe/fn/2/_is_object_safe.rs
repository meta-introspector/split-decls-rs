use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: _is_object_safe");
fn _is_object_safe (_ : & dyn DoubleEndedFallibleIterator < Item = () , Error = () >) { }
}