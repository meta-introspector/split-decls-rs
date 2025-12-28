use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P4_Mul_N5 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type N20 = NInt < UInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P4MulN5 = < < A as Mul < B > > :: Output as Same < N20 > > :: Output ; assert_eq ! (< P4MulN5 as Integer >:: to_i64 () , < N20 as Integer >:: to_i64 ()) ; }