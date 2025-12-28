use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test__0_Neg () { type A = Z0 ; type _0 = Z0 ; # [allow (non_camel_case_types)] type Neg_0 = < < A as Neg > :: Output as Same < _0 > > :: Output ; assert_eq ! (< Neg_0 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }