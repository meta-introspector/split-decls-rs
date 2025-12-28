use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N1_Div_N5 () { type A = NInt < UInt < UTerm , B1 > > ; type B = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type N1DivN5 = < < A as Div < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< N1DivN5 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }