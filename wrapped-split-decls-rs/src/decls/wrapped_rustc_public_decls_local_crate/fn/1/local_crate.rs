use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Access to the local crate."] pub fn local_crate () -> Crate { with (| cx | cx . local_crate ()) }
}