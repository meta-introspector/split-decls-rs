use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: respan");
pub fn respan < T > (sp : Span , t : T) -> Spanned < T > { Spanned { node : t , span : sp } }
}