use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N3_Mul__0 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = Z0 ; type _0 = Z0 ; # [allow (non_camel_case_types)] type N3Mul_0 = < < A as Mul < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< N3Mul_0 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }