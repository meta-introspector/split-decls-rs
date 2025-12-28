use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_cell () { use std :: cell :: { Cell , RefCell } ; # [derive (Encodable_NoContext , Decodable_NoContext , PartialEq , Debug)] struct A { baz : isize , } # [derive (Encodable_NoContext , Decodable_NoContext , PartialEq , Debug)] struct B { foo : Cell < bool > , bar : RefCell < A > , } let obj = B { foo : Cell :: new (true) , bar : RefCell :: new (A { baz : 2 }) } ; check_round_trip (vec ! [obj]) ; }