use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
static mut CALL_TREE : Vec < CallTrace > = Vec :: new () ;
}