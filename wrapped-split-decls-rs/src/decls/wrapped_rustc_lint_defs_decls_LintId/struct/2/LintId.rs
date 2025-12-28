use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Identifies a lint known to the compiler."] # [derive (Clone , Copy , Debug)] pub struct LintId { pub lint : & 'static Lint , }
}