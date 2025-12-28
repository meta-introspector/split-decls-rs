use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N4_Gcd_N4 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P4 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N4GcdN4 = < < A as Gcd < B > > :: Output as Same < P4 > > :: Output ; assert_eq ! (< N4GcdN4 as Integer >:: to_i64 () , < P4 as Integer >:: to_i64 ()) ; }