use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn test_box () { # [derive (Encodable_NoContext , Decodable_NoContext , PartialEq , Debug)] struct A { foo : Box < [bool] > , } let obj = A { foo : Box :: new ([true , false]) } ; check_round_trip (vec ! [obj]) ; }
}