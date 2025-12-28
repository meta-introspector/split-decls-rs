use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type InternMap < T > = DashMap < Arc < T > , () , BuildHasherDefault < FxHasher > > ;
}