use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct WorkflowExecutor { verbose : bool , dry_run : bool , global_config : SplitDeclsConfig , context : HashMap < String , toml :: Value > , }
}