use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Error from the `homogeneous_aggregate` test function, indicating"] # [doc = " there are distinct leaf fields passed in different ways,"] # [doc = " or this is uninhabited."] # [derive (Copy , Clone , Debug)] pub struct Heterogeneous ;