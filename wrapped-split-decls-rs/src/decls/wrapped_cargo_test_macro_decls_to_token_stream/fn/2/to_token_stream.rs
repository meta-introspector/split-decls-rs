use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: to_token_stream");
fn to_token_stream (code : & str) -> TokenStream { code . parse () . unwrap () }
}