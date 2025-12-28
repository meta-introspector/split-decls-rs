use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn is_sorted < T : Send + Ord > (v : & [T]) -> bool { (1 .. v . len ()) . all (| i | v [i - 1] <= v [i]) }
}