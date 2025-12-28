use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N2_Div_P2 () { type A = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type N1 = NInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N2DivP2 = < < A as Div < B > > :: Output as Same < N1 > > :: Output ; assert_eq ! (< N2DivP2 as Integer >:: to_i64 () , < N1 as Integer >:: to_i64 ()) ; }