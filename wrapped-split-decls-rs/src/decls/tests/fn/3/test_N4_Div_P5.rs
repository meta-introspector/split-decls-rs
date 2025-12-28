use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N4_Div_P5 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type N4DivP5 = < < A as Div < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< N4DivP5 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }