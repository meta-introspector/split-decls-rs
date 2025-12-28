use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P1_Mul_N5 () { type A = PInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N5 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type P1MulN5 = < < A as Mul < B > > :: Output as Same < N5 > > :: Output ; assert_eq ! (< P1MulN5 as Integer >:: to_i64 () , < N5 as Integer >:: to_i64 ()) ; }