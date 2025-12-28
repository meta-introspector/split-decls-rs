use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
unsafe impl < L : Sync > Sync for LatchRef < '_ , L > { }
}