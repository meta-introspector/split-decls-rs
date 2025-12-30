use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: is_ci");
# [doc = " Whether or not this running in a Continuous Integration environment."] pub fn is_ci () -> bool { std :: env :: var ("CI") . is_ok () || std :: env :: var ("TF_BUILD") . is_ok () }
}