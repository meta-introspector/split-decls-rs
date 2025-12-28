use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P3_Mul_P4 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P12 = PInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P3MulP4 = < < A as Mul < B > > :: Output as Same < P12 > > :: Output ; assert_eq ! (< P3MulP4 as Integer >:: to_i64 () , < P12 as Integer >:: to_i64 ()) ; }