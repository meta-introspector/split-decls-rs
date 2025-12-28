use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct AttrIdGenerator (AtomicU32) ;
}