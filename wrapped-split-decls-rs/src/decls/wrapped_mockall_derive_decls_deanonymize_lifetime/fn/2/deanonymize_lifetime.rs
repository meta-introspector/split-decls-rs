use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: deanonymize_lifetime");
fn deanonymize_lifetime (lt : & mut Lifetime) { if lt . ident == "_" { lt . ident = format_ident ! ("static") ; } }
}