use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P5_Mul_N5 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N25 = NInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type P5MulN5 = < < A as Mul < B > > :: Output as Same < N25 > > :: Output ; assert_eq ! (< P5MulN5 as Integer >:: to_i64 () , < N25 as Integer >:: to_i64 ()) ; }