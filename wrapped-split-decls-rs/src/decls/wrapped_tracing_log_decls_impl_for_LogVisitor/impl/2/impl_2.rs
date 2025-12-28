use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > LogVisitor < 'a > { fn new_for (_event : & 'a Event < 'a > , fields : & 'static Fields) -> Self { Self { target : None , module_path : None , file : None , line : None , fields , } } }
}