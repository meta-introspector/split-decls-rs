use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl AutoDiffAttrs { pub fn has_primal_ret (& self) -> bool { matches ! (self . ret_activity , DiffActivity :: Active | DiffActivity :: Dual) } }
}