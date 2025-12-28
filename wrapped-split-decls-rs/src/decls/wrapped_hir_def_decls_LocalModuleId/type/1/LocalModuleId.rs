use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " An ID of a module, **local** to a `DefMap`."] pub type LocalModuleId = Idx < nameres :: ModuleData > ;