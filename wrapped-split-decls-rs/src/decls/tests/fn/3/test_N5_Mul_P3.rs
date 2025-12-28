use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N5_Mul_P3 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N15 = NInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N5MulP3 = < < A as Mul < B > > :: Output as Same < N15 > > :: Output ; assert_eq ! (< N5MulP3 as Integer >:: to_i64 () , < N15 as Integer >:: to_i64 ()) ; }