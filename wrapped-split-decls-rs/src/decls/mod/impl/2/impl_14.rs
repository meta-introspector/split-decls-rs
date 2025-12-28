use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [doc = " The empty annotation, which does nothing."] impl Annotation for () { fn merge_reached (self , _other : Self) -> Self { () } fn merge_scc (self , _other : Self) -> Self { () } }
}