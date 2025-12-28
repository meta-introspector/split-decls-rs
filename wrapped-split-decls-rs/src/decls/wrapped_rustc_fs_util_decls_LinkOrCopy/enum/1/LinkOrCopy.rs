use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
pub enum LinkOrCopy { Link , Copy , }
}