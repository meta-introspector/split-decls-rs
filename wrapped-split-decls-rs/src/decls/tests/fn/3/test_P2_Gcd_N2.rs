use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P2_Gcd_N2 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P2 = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P2GcdN2 = < < A as Gcd < B > > :: Output as Same < P2 > > :: Output ; assert_eq ! (< P2GcdN2 as Integer >:: to_i64 () , < P2 as Integer >:: to_i64 ()) ; }