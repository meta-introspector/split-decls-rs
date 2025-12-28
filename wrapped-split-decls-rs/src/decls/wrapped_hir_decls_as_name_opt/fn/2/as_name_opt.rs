use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: as_name_opt");
fn as_name_opt (name : Option < impl AsName >) -> Name { name . map_or_else (Name :: missing , | name | name . as_name ()) }
}