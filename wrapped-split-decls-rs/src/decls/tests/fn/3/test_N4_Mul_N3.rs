use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N4_Mul_N3 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P12 = PInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N4MulN3 = < < A as Mul < B > > :: Output as Same < P12 > > :: Output ; assert_eq ! (< N4MulN3 as Integer >:: to_i64 () , < P12 as Integer >:: to_i64 ()) ; }