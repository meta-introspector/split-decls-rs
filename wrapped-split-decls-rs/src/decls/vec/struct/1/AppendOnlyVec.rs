use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Default)] pub struct AppendOnlyVec < T : Copy > { vec : parking_lot :: RwLock < Vec < T > > , }
}