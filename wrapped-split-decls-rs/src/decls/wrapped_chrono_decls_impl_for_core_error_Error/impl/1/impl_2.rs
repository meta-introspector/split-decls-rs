use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (all (not (feature = "std") , feature = "core-error"))] impl core :: error :: Error for OutOfRange { }
}