use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T1 : StableOrd , T2 : StableOrd > StableOrd for (T1 , T2) { const CAN_USE_UNSTABLE_SORT : bool = T1 :: CAN_USE_UNSTABLE_SORT && T2 :: CAN_USE_UNSTABLE_SORT ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }
}