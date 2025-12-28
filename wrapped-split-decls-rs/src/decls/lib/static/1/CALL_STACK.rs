use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstatic! {
static mut CALL_STACK : Vec < String > = Vec :: new () ;
}