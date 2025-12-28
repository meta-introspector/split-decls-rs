use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [allow (non_snake_case)] # [test] fn lookup_Rust () { let abi = ExternAbi :: from_str ("Rust") ; assert ! (abi . is_ok () && abi . unwrap () . as_str () == "Rust") ; }