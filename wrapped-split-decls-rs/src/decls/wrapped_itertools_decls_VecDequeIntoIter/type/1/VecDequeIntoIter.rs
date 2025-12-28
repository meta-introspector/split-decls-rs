use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [cfg (feature = "use_alloc")] type VecDequeIntoIter < T > = alloc :: collections :: vec_deque :: IntoIter < T > ;
}