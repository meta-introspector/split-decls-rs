use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
enum HashStableMode { Normal , Generic , NoContext , }
}