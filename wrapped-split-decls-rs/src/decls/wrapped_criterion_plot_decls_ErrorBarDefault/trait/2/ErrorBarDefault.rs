use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Error bar variant of Default"] trait ErrorBarDefault < S > { # [doc = " Creates `errorbar::Properties` with default configuration"] fn default (s : S) -> Self ; }