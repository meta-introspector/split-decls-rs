use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [allow (non_snake_case)] fn test__0_Sub__0 () { type A = Z0 ; type B = Z0 ; type _0 = Z0 ; # [allow (non_camel_case_types)] type _0Sub_0 = < < A as Sub < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< _0Sub_0 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }
}