use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N3_Mul_N2 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P6 = PInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N3MulN2 = < < A as Mul < B > > :: Output as Same < P6 > > :: Output ; assert_eq ! (< N3MulN2 as Integer >:: to_i64 () , < P6 as Integer >:: to_i64 ()) ; }