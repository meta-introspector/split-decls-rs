use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
static mut CALL_COUNTER : u64 = 0 ;
}