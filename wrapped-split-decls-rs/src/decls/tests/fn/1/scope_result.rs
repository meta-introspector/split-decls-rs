use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn scope_result () { let x = scope (| _ | 22) ; assert_eq ! (x , 22) ; }