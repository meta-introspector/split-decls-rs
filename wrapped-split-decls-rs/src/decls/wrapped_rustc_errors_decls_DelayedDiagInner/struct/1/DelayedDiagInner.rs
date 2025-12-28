use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct DelayedDiagInner { inner : DiagInner , note : Backtrace , }
}