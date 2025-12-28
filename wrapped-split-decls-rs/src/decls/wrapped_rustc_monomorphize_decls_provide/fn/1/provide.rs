use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn provide (providers : & mut Providers) { partitioning :: provide (providers) ; mono_checks :: provide (providers) ; }