use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn from_cstr () { let c = c"foo" ; let s : SmallCStr = c . into () ; assert_eq ! (s . len_with_nul () , 4) ; assert_eq ! (s . as_c_str () , c"foo") ; }
}