use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test__0_Gcd_P5 () { type A = Z0 ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P5 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type _0GcdP5 = < < A as Gcd < B > > :: Output as Same < P5 > > :: Output ; assert_eq ! (< _0GcdP5 as Integer >:: to_i64 () , < P5 as Integer >:: to_i64 ()) ; }