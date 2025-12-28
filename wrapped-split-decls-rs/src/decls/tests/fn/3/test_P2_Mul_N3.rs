use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P2_Mul_N3 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N6 = NInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P2MulN3 = < < A as Mul < B > > :: Output as Same < N6 > > :: Output ; assert_eq ! (< P2MulN3 as Integer >:: to_i64 () , < N6 as Integer >:: to_i64 ()) ; }