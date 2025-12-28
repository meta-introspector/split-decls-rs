use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P5_Div_N1 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type B = NInt < UInt < UTerm , B1 > > ; type N5 = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type P5DivN1 = < < A as Div < B > > :: Output as Same < N5 > > :: Output ; assert_eq ! (< P5DivN1 as Integer >:: to_i64 () , < N5 as Integer >:: to_i64 ()) ; }