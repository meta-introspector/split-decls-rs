use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (not (feature = "std"))] impl core :: error :: Error for Errors { }
}