use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test__0_Max_P1 () { type A = Z0 ; type B = PInt < UInt < UTerm , B1 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type _0MaxP1 = < < A as Max < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< _0MaxP1 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }