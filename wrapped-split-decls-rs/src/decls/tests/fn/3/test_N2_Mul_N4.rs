use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N2_Mul_N4 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P8 = PInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type N2MulN4 = < < A as Mul < B > > :: Output as Same < P8 > > :: Output ; assert_eq ! (< N2MulN4 as Integer >:: to_i64 () , < P8 as Integer >:: to_i64 ()) ; }