use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N5_Mul_N4 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P20 = PInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N5MulN4 = < < A as Mul < B > > :: Output as Same < P20 > > :: Output ; assert_eq ! (< N5MulN4 as Integer >:: to_i64 () , < P20 as Integer >:: to_i64 ()) ; }