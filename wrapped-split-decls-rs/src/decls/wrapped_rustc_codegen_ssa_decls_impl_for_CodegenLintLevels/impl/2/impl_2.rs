use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl CodegenLintLevels { pub fn from_tcx (tcx : TyCtxt < '_ >) -> Self { Self { linker_messages : tcx . lint_level_at_node (LINKER_MESSAGES , CRATE_HIR_ID) , } } }