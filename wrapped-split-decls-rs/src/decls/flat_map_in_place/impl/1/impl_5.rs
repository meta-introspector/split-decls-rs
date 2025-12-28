use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < T > FlatMapInPlace < T > for Vec < T > { flat_map_in_place ! (Vec) ; }