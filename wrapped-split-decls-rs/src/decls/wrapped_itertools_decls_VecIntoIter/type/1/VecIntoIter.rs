use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg (feature = "use_alloc")] type VecIntoIter < T > = alloc :: vec :: IntoIter < T > ;