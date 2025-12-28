use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T : StableOrd > StableOrd for Option < T > { const CAN_USE_UNSTABLE_SORT : bool = T :: CAN_USE_UNSTABLE_SORT ; const THIS_IMPLEMENTATION_HAS_BEEN_TRIPLE_CHECKED : () = () ; }