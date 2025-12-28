use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test__0_Add__0 () { type A = Z0 ; type B = Z0 ; type _0 = Z0 ; # [allow (non_camel_case_types)] type _0Add_0 = < < A as Add < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< _0Add_0 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }