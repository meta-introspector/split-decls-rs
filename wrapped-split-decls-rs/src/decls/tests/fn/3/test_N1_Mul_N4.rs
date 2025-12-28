use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N1_Mul_N4 () { type A = NInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P4 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N1MulN4 = < < A as Mul < B > > :: Output as Same < P4 > > :: Output ; assert_eq ! (< N1MulN4 as Integer >:: to_i64 () , < P4 as Integer >:: to_i64 ()) ; }