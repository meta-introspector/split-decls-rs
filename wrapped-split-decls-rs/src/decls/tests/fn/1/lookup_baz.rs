use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn lookup_baz () { let abi = ExternAbi :: from_str ("baz") ; assert_matches ! (abi , Err (AbiFromStrErr :: Unknown)) ; }