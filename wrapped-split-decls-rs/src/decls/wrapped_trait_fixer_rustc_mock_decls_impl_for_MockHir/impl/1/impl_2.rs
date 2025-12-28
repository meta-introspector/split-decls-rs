use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl MockHir { pub fn walk_tops (self , _f : impl FnMut (& Item < 'static >)) { } }
}