use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl PartialEq for LintId { fn eq (& self , other : & LintId) -> bool { std :: ptr :: eq (self . lint , other . lint) } }