use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Default)] pub struct RwLock < T > (parking_lot :: RwLock < T >) ;
}