use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
const FULL_DISTANCES : usize = 1 << (DIST_MODEL_END / 2) ;
}