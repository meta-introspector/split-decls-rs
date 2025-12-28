use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N3_Gcd_P4 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N3GcdP4 = < < A as Gcd < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< N3GcdP4 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }