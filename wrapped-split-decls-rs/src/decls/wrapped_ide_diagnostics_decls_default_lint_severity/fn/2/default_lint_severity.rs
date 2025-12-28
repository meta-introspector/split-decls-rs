use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn default_lint_severity (lint : & Lint , edition : Edition) -> Severity { if lint . deny_since . is_some_and (| e | edition >= e) { Severity :: Error } else if lint . warn_since . is_some_and (| e | edition >= e) { Severity :: Warning } else { lint . default_severity } }
}