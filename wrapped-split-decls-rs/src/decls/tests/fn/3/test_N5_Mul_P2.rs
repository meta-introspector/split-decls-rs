use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N5_Mul_P2 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type N10 = NInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N5MulP2 = < < A as Mul < B > > :: Output as Same < N10 > > :: Output ; assert_eq ! (< N5MulP2 as Integer >:: to_i64 () , < N10 as Integer >:: to_i64 ()) ; }