use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " An ID of a module, **local** to a `DefMap`."] pub type LocalModuleId = Idx < nameres :: ModuleData > ;
}