use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn lookup_cdecl () { let abi = ExternAbi :: from_str ("cdecl") ; assert ! (abi . is_ok () && abi . unwrap () . as_str () == "cdecl") ; }