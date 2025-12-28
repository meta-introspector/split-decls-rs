use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [must_use] pub fn get_commit_hash () -> Option < String > { let mut stdout = get_output ("git" , & ["rev-parse" , "HEAD"]) ? ; stdout . truncate (10) ; Some (stdout) }
}