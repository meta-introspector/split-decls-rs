use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn to_token_stream (code : & str) -> TokenStream { code . parse () . unwrap () }