use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn static_lt () -> Lifetime { Lifetime :: new ("'static" , Span :: call_site ()) }
}