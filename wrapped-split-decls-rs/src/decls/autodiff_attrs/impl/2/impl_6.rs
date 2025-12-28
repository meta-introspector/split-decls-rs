use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl DiffActivity { pub fn is_dual_or_const (& self) -> bool { use DiffActivity :: * ; matches ! (self , | Dual | DualOnly | Dualv | DualvOnly | Const) } }
}