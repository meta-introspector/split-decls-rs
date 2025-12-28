use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PartialEq for FnAbi { fn eq (& self , _other : & Self) -> bool { true } }
}