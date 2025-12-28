use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn apply_single_fix (toml : & mut Value , issue : & DependencyIssue , fix : & str) -> Result < () > { println ! ("  Would fix {} -> {}: {}" , issue . crate_name , issue . dep_name , fix) ; Ok (()) }