use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N2_Gcd__0 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = Z0 ; type P2 = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N2Gcd_0 = < < A as Gcd < B > > :: Output as Same < P2 > > :: Output ; assert_eq ! (< N2Gcd_0 as Integer >:: to_i64 () , < P2 as Integer >:: to_i64 ()) ; }