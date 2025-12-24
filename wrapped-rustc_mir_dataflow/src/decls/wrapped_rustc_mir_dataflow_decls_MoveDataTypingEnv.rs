use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub struct MoveDataTypingEnv<'tcx> {
    pub move_data: MoveData<'tcx>,
    pub typing_env: ty::TypingEnv<'tcx>,
}
