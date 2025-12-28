use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T > ViaNothing for ArgPrinter < '_ , T > { fn debug_string (& self) -> NothingPrint { NothingPrint } }
}