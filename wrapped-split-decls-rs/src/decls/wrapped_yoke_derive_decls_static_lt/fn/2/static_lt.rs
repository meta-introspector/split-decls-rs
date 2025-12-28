use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: static_lt");
fn static_lt () -> Lifetime { Lifetime :: new ("'static" , Span :: call_site ()) }
}