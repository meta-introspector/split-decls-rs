use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl DwimIntent { fn hash (& self) -> String { format ! ("{:x}" , self . keywords . join ("") . len ()) } }
}