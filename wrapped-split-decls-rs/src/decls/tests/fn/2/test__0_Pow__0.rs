use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test__0_Pow__0 () { type A = Z0 ; type B = Z0 ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type _0Pow_0 = < < A as Pow < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< _0Pow_0 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }