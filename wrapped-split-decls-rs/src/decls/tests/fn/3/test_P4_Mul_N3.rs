use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P4_Mul_N3 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N12 = NInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P4MulN3 = < < A as Mul < B > > :: Output as Same < N12 > > :: Output ; assert_eq ! (< P4MulN3 as Integer >:: to_i64 () , < N12 as Integer >:: to_i64 ()) ; }