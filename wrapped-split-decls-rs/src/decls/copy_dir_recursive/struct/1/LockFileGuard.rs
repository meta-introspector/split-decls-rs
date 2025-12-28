use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct LockFileGuard { path : PathBuf , }