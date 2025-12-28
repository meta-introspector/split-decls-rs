use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
pub enum MixedBitIter < 'a , T : Idx > { Small (BitIter < 'a , T >) , Large (ChunkedBitIter < 'a , T >) , }
}