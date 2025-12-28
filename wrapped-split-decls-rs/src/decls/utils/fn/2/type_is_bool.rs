use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Checks whether the type `ty` is `bool`."] pub (crate) fn type_is_bool (ty : & Type) -> bool { type_matches_path (ty , & ["bool"]) }
}