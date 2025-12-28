use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn test_bool () { check_round_trip (vec ! [false , true , true , false , false]) ; }