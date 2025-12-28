use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl StableCompare for Symbol { const CAN_USE_UNSTABLE_SORT : bool = true ; fn stable_cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . as_str () . cmp (other . as_str ()) } }