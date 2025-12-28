use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P5_Mul_N2 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type N10 = NInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P5MulN2 = < < A as Mul < B > > :: Output as Same < N10 > > :: Output ; assert_eq ! (< P5MulN2 as Integer >:: to_i64 () , < N10 as Integer >:: to_i64 ()) ; }