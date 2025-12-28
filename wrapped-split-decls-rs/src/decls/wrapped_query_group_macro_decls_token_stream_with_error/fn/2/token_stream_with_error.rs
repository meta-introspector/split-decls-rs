use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub (crate) fn token_stream_with_error (mut tokens : TokenStream , error : syn :: Error) -> TokenStream { tokens . extend (TokenStream :: from (error . into_compile_error ())) ; tokens }