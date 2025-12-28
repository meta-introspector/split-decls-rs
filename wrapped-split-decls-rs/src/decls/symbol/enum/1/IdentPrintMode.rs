use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
pub enum IdentPrintMode { Normal , RawIdent , RawLifetime , }
}