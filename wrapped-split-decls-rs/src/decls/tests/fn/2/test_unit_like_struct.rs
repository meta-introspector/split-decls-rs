use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_unit_like_struct () { # [derive (Encodable_NoContext , Decodable_NoContext , PartialEq , Debug)] struct UnitLikeStruct ; check_round_trip (vec ! [UnitLikeStruct]) ; }
}