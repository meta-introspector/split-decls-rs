use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn ty_to_string (ann : & dyn PpAnn , ty : & hir :: Ty < '_ >) -> String { to_string (ann , | s | s . print_type (ty)) }