use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , PartialEq , Eq , Debug)] enum TryReserveErrorKind { Std (alloc :: collections :: TryReserveError) , CapacityOverflow , AllocError { layout : alloc :: alloc :: Layout } , }