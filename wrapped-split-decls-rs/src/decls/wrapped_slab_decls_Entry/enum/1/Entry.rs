use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone)] enum Entry < T > { Vacant (usize) , Occupied (T) , }