use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: dummy_spanned");
pub fn dummy_spanned < T > (t : T) -> Spanned < T > { respan (DUMMY_SP , t) }
}