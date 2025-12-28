use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn to_token_stream (code : & str) -> TokenStream { code . parse () . unwrap () }
}