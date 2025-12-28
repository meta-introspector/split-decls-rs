use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : AutoFinish > Drop for AutoFinisher < T > { fn drop (& mut self) { if let Some (writer) = self . 0 . take () { writer . finish_ignore_error () ; } } }
}