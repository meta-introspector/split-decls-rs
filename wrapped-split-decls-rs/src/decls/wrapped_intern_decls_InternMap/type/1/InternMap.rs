use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type InternMap < T > = DashMap < Arc < T > , () , BuildHasherDefault < FxHasher > > ;