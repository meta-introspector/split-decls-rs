use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct EvaluatedConst < 'db > { def : DefWithBodyId , const_ : hir_ty :: next_solver :: Const < 'db > , ty : Ty < 'db > , }