use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct BootstrapTracer { pub trace : BootstrapTrace , pub current_state : Option < String > , }
}