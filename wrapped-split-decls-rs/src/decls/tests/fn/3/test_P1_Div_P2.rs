use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P1_Div_P2 () { type A = PInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type P1DivP2 = < < A as Div < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< P1DivP2 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }