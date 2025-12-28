use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static mut CALL_TREE : Vec < CallTrace > = Vec :: new () ;