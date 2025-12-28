use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N2_Mul_N5 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P10 = PInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N2MulN5 = < < A as Mul < B > > :: Output as Same < P10 > > :: Output ; assert_eq ! (< N2MulN5 as Integer >:: to_i64 () , < P10 as Integer >:: to_i64 ()) ; }