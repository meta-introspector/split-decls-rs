use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N4_Mul_P5 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N20 = NInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N4MulP5 = < < A as Mul < B > > :: Output as Same < N20 > > :: Output ; assert_eq ! (< N4MulP5 as Integer >:: to_i64 () , < N20 as Integer >:: to_i64 ()) ; }