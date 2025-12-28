use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Drop for TempDir { fn drop (& mut self) { if self . delete_on_drop { let result = Self :: remove_dir (& self . path_buf) ; if self . panic_on_delete_err { if let Err (e) = result { panic ! ("{}" , e) ; } } } } }
}