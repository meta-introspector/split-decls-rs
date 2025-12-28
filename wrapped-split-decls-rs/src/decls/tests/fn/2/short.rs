use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn short () { const TEXT : & str = "abcd" ; let reference = ffi :: CString :: new (TEXT . to_string ()) . unwrap () ; let scs = SmallCStr :: new (TEXT) ; assert_eq ! (scs . len_with_nul () , TEXT . len () + 1) ; assert_eq ! (scs . as_c_str () , reference . as_c_str ()) ; assert ! (! scs . spilled ()) ; }