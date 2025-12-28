use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P3_Mul_N1 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UTerm , B1 > > ; type N3 = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P3MulN1 = < < A as Mul < B > > :: Output as Same < N3 > > :: Output ; assert_eq ! (< P3MulN1 as Integer >:: to_i64 () , < N3 as Integer >:: to_i64 ()) ; }