use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct RustcVersion { major : u16 , minor : u16 , patch : u16 , }
}