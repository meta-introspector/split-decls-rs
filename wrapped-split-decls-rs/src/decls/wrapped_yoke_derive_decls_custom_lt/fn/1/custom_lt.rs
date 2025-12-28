use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn custom_lt (s : & str) -> Lifetime { Lifetime :: new (s , Span :: call_site ()) }