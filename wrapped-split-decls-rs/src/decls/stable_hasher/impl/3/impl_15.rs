use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [doc = " `StableOrd` denotes that the type's `Ord` implementation is stable, so"] # [doc = " we can implement `StableCompare` by just delegating to `Ord`."] impl < T : StableOrd > StableCompare for T { const CAN_USE_UNSTABLE_SORT : bool = T :: CAN_USE_UNSTABLE_SORT ; fn stable_cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . cmp (other) } }
}