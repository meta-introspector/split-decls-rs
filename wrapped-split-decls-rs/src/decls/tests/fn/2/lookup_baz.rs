use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: lookup_baz");
# [test] fn lookup_baz () { let abi = ExternAbi :: from_str ("baz") ; assert_matches ! (abi , Err (AbiFromStrErr :: Unknown)) ; }
}