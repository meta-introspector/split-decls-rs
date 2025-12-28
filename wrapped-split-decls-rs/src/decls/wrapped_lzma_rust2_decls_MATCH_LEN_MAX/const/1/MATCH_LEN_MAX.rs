use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
const MATCH_LEN_MAX : usize = MATCH_LEN_MIN + LOW_SYMBOLS + MID_SYMBOLS + HIGH_SYMBOLS - 1 ;
}