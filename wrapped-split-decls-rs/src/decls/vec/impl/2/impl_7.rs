use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < A : Copy > FromIterator < A > for AppendOnlyVec < A > { fn from_iter < T : IntoIterator < Item = A > > (iter : T) -> Self { let this = Self :: new () ; for val in iter { this . push (val) ; } this } }