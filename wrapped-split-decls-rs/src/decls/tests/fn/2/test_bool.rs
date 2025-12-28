use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_bool");
# [test] fn test_bool () { check_round_trip (vec ! [false , true , true , false , false]) ; }
}