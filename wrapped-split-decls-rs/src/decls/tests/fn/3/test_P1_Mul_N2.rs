use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P1_Mul_N2 () { type A = PInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P1MulN2 = < < A as Mul < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< P1MulN2 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }