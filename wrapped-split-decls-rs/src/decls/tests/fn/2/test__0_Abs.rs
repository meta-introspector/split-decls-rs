use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test__0_Abs () { type A = Z0 ; type _0 = Z0 ; # [allow (non_camel_case_types)] type Abs_0 = < < A as Abs > :: Output as Same < _0 > > :: Output ; assert_eq ! (< Abs_0 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }