use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn long () { const TEXT : & str = "01234567890123456789012345678901234567890123456789\
                        01234567890123456789012345678901234567890123456789\
                        01234567890123456789012345678901234567890123456789" ; let reference = ffi :: CString :: new (TEXT . to_string ()) . unwrap () ; let scs = SmallCStr :: new (TEXT) ; assert_eq ! (scs . len_with_nul () , TEXT . len () + 1) ; assert_eq ! (scs . as_c_str () , reference . as_c_str ()) ; assert ! (scs . spilled ()) ; }