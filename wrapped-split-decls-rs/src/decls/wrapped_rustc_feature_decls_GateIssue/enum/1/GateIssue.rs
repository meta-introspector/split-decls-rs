use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
pub enum GateIssue { Language , Library (Option < NonZero < u32 > >) , }
}