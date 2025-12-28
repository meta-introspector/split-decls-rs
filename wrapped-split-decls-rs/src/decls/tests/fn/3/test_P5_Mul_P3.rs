use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P5_Mul_P3 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P15 = PInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P5MulP3 = < < A as Mul < B > > :: Output as Same < P15 > > :: Output ; assert_eq ! (< P5MulP3 as Integer >:: to_i64 () , < P15 as Integer >:: to_i64 ()) ; }