use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Ord for Span { fn cmp (& self , rhs : & Self) -> Ordering { Ord :: cmp (& self . data () , & rhs . data ()) } }
}