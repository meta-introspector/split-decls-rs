use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test__0_Mul_N3 () { type A = Z0 ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type _0MulN3 = < < A as Mul < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< _0MulN3 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }