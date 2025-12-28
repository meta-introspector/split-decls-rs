use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N1_Mul_P1 () { type A = NInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UTerm , B1 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N1MulP1 = < < A as Mul < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< N1MulP1 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }