use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PartialEq for S { fn eq (& self , _other : & Self) -> bool { panic ! ("shouldn't be called") ; } }
}