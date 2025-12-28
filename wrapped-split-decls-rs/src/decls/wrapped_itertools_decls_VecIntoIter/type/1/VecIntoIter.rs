use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [cfg (feature = "use_alloc")] type VecIntoIter < T > = alloc :: vec :: IntoIter < T > ;
}