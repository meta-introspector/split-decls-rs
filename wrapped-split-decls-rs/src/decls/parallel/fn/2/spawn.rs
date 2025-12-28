use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn spawn (func : impl FnOnce () + DynSend + 'static) { if mode :: is_dyn_thread_safe () { let func = FromDyn :: from (func) ; rustc_thread_pool :: spawn (| | { (func . into_inner ()) () ; }) ; } else { func () } }