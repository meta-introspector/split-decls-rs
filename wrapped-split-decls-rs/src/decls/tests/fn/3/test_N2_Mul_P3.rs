use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N2_Mul_P3 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N6 = NInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N2MulP3 = < < A as Mul < B > > :: Output as Same < N6 > > :: Output ; assert_eq ! (< N2MulP3 as Integer >:: to_i64 () , < N6 as Integer >:: to_i64 ()) ; }