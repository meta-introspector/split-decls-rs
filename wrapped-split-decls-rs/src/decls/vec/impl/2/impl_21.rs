use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I : Idx , T , const N : usize > From < [T ; N] > for IndexVec < I , T > { # [inline] fn from (array : [T ; N]) -> Self { IndexVec :: from_raw (array . into ()) } }