use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P4_Mul_N2 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type N8 = NInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P4MulN2 = < < A as Mul < B > > :: Output as Same < N8 > > :: Output ; assert_eq ! (< P4MulN2 as Integer >:: to_i64 () , < N8 as Integer >:: to_i64 ()) ; }