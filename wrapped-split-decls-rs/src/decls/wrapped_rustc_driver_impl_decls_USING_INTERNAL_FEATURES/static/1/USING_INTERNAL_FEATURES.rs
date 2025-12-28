use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
pub static USING_INTERNAL_FEATURES : AtomicBool = AtomicBool :: new (false) ;
}