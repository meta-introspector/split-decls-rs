use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub struct MockTyCtxt<'tcx>(pub TyCtxt<'tcx>);
