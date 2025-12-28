use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N2_Gcd_P5 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N2GcdP5 = < < A as Gcd < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< N2GcdP5 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }