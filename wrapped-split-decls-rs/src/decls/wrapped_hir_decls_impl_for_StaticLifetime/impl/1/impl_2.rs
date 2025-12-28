use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl StaticLifetime { pub fn name (self) -> Name { Name :: new_symbol_root (sym :: tick_static) } }
}