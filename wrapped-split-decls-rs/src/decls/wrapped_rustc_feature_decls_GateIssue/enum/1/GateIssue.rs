use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub enum GateIssue { Language , Library (Option < NonZero < u32 > >) , }