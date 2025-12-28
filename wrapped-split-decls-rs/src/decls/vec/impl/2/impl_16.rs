use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I : Idx , T > FromIterator < T > for IndexVec < I , T > { # [inline] fn from_iter < J > (iter : J) -> Self where J : IntoIterator < Item = T > , { IndexVec :: from_raw (Vec :: from_iter (iter)) } }