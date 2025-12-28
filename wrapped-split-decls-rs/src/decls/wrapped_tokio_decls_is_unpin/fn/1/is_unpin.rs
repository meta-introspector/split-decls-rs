use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [cfg (feature = "io-util")] # [cfg (test)] fn is_unpin < T : Unpin > () { }
}