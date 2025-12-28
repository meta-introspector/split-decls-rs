use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test__0_Pow_P5 () { type A = Z0 ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type _0PowP5 = < < A as Pow < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< _0PowP5 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }