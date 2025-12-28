use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N3_Div_P1 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UTerm , B1 > > ; type N3 = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N3DivP1 = < < A as Div < B > > :: Output as Same < N3 > > :: Output ; assert_eq ! (< N3DivP1 as Integer >:: to_i64 () , < N3 as Integer >:: to_i64 ()) ; }