use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P1_Mul_N1 () { type A = PInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UTerm , B1 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type P1MulN1 = < < A as Mul < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< P1MulN1 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }