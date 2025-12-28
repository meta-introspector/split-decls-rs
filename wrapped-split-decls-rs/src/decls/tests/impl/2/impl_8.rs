use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Annotation for MaxReached { fn merge_scc (self , other : Self) -> Self { Self (std :: cmp :: max (other . 0 , self . 0)) } fn merge_reached (self , other : Self) -> Self { Self (std :: cmp :: max (other . 0 , self . 0)) } }
}