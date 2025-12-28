use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct BuiltLint { lint : & 'static Lint , groups : Vec < & 'static str > , }