use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N4_Max_P1 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UTerm , B1 > > ; type P1 = PInt < UInt < UTerm , B1 > > ; # [allow (non_camel_case_types)] type N4MaxP1 = < < A as Max < B > > :: Output as Same < P1 > > :: Output ; assert_eq ! (< N4MaxP1 as Integer >:: to_i64 () , < P1 as Integer >:: to_i64 ()) ; }