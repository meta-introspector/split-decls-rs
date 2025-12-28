use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn get_key (data : & Element) -> usize { data . 0 }
}