use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P4_Gcd__0 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = Z0 ; type P4 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P4Gcd_0 = < < A as Gcd < B > > :: Output as Same < P4 > > :: Output ; assert_eq ! (< P4Gcd_0 as Integer >:: to_i64 () , < P4 as Integer >:: to_i64 ()) ; }