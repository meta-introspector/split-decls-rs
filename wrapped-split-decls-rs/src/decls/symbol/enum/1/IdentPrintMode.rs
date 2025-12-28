use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub enum IdentPrintMode { Normal , RawIdent , RawLifetime , }