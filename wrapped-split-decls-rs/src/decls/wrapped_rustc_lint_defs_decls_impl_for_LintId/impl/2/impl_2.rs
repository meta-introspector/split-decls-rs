use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl LintId { # [doc = " Gets the `LintId` for a `Lint`."] pub fn of (lint : & 'static Lint) -> LintId { LintId { lint } } pub fn lint_name_raw (& self) -> & 'static str { self . lint . name } # [doc = " Gets the name of the lint."] pub fn to_string (& self) -> String { self . lint . name_lower () } }