use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N3_Gcd_P3 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P3 = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N3GcdP3 = < < A as Gcd < B > > :: Output as Same < P3 > > :: Output ; assert_eq ! (< N3GcdP3 as Integer >:: to_i64 () , < P3 as Integer >:: to_i64 ()) ; }