use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: custom_lt");
fn custom_lt (s : & str) -> Lifetime { Lifetime :: new (s , Span :: call_site ()) }
}