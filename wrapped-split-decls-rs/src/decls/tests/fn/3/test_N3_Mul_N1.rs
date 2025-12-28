use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N3_Mul_N1 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UTerm , B1 > > ; type P3 = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N3MulN1 = < < A as Mul < B > > :: Output as Same < P3 > > :: Output ; assert_eq ! (< N3MulN1 as Integer >:: to_i64 () , < P3 as Integer >:: to_i64 ()) ; }