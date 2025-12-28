use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl StableOrd for bool { const CAN_USE_UNSTABLE_SORT : bool = true ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }
}