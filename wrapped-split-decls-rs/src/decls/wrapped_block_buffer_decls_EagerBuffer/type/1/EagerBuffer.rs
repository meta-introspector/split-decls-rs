use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " Eager block buffer."] pub type EagerBuffer < B > = BlockBuffer < B , Eager > ;
}