use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P2_Mul_P4 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P8 = PInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P2MulP4 = < < A as Mul < B > > :: Output as Same < P8 > > :: Output ; assert_eq ! (< P2MulP4 as Integer >:: to_i64 () , < P8 as Integer >:: to_i64 ()) ; }