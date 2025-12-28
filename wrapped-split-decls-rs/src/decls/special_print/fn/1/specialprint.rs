use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn specialprint (macro_name : & str , local_score : f64) { print ! (", {}: {:.4}" , macro_name , local_score) ; }
}