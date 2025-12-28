use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct LockFileGuard { path : PathBuf , }
}