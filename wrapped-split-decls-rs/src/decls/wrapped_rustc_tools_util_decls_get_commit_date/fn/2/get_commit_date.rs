use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [must_use] pub fn get_commit_date () -> Option < String > { get_output ("git" , & ["log" , "-1" , "--date=short" , "--pretty=format:%cd"]) }