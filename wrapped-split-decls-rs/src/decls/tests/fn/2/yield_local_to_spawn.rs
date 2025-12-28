use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn yield_local_to_spawn () { let (tx , rx) = channel () ; crate :: spawn (move | | tx . send (22) . unwrap ()) ; crate :: registry :: in_worker (move | _ , _ | { crate :: yield_local () ; }) ; assert_eq ! (22 , rx . recv () . unwrap ()) ; }