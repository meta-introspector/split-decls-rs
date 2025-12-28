use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: test_unit");
# [test] fn test_unit () { check_round_trip (vec ! [() , () , () , ()]) ; }
}