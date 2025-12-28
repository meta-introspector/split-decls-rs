use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N2_Mul_P2 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type N4 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N2MulP2 = < < A as Mul < B > > :: Output as Same < N4 > > :: Output ; assert_eq ! (< N2MulP2 as Integer >:: to_i64 () , < N4 as Integer >:: to_i64 ()) ; }