use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [track_caller] pub fn assert_deterministic_mtime (path : impl AsRef < Path >) { const DETERMINISTIC_TIMESTAMP : u64 = 1153704088 ; let path = path . as_ref () ; let mtime = path . metadata () . unwrap () . modified () . unwrap () ; let timestamp = mtime . duration_since (std :: time :: UNIX_EPOCH) . unwrap () . as_secs () ; assert_eq ! (timestamp , DETERMINISTIC_TIMESTAMP , "expected deterministic mtime for {path:?}, got {timestamp}") ; }
}