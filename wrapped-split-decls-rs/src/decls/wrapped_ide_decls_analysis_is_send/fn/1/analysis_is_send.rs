use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn analysis_is_send () { fn is_send < T : Send > () { } is_send :: < Analysis > () ; }
}