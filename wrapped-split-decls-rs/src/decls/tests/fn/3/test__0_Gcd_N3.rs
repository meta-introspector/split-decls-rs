use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test__0_Gcd_N3 () { type A = Z0 ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P3 = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type _0GcdN3 = < < A as Gcd < B > > :: Output as Same < P3 > > :: Output ; assert_eq ! (< _0GcdN3 as Integer >:: to_i64 () , < P3 as Integer >:: to_i64 ()) ; }