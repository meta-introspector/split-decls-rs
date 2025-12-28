use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub enum MixedBitIter < 'a , T : Idx > { Small (BitIter < 'a , T >) , Large (ChunkedBitIter < 'a , T >) , }