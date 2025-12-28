use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N1_Gcd_P3 () { type A = NInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N1GcdP3 = < < A as Gcd < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< N1GcdP3 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }