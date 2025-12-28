use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N1_Add_P1 () { type A = NInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UTerm , B1 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type N1AddP1 = < < A as Add < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< N1AddP1 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }