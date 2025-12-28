use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > Ord for Idx < T > { fn cmp (& self , other : & Self) -> cmp :: Ordering { self . raw . cmp (& other . raw) } }
}