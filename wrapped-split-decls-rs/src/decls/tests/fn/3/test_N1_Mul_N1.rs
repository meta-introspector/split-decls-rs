use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N1_Mul_N1 () { type A = NInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UTerm , B1 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N1MulN1 = < < A as Mul < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< N1MulN1 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }