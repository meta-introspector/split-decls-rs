use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc (hidden)] pub trait ViaDebug < T > where T : Debug , { fn debug_string (& self) -> DebugPrint < '_ , T > ; }