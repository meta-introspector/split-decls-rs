use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [cfg (not (feature = "regex_enabled"))] pub type CurrentRegexMatcher = DummyRegexMatcher ;
}