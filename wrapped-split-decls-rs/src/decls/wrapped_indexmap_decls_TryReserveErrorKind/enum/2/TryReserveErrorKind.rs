use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , PartialEq , Eq , Debug)] enum TryReserveErrorKind { Std (alloc :: collections :: TryReserveError) , CapacityOverflow , AllocError { layout : alloc :: alloc :: Layout } , }
}