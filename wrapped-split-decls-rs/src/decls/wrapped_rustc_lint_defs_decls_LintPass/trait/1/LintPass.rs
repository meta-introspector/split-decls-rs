use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait LintPass { fn name (& self) -> & 'static str ; fn get_lints (& self) -> LintVec ; }