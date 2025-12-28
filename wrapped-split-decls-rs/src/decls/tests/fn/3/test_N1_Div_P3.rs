use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N1_Div_P3 () { type A = NInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type N1DivP3 = < < A as Div < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< N1DivP3 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }