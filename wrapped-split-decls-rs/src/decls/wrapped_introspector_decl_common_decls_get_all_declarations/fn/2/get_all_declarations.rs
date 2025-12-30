use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: get_all_declarations");
pub fn get_all_declarations () -> Vec < DeclInfo > { DECL_REGISTRY . lock () . map (| r | r . declarations . clone ()) . unwrap_or_default () }
}