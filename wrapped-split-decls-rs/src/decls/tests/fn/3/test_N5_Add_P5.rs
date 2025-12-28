use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N5_Add_P5 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type _0 = Z0 ; # [allow (non_camel_case_types)] type N5AddP5 = < < A as Add < B > > :: Output as Same < _0 > > :: Output ; assert_eq ! (< N5AddP5 as Integer >:: to_i64 () , < _0 as Integer >:: to_i64 ()) ; }