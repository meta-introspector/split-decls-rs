use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > FlatMapInPlace < T > for ThinVec < T > { flat_map_in_place ! (ThinVec) ; }