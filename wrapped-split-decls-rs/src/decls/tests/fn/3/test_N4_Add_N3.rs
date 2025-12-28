use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N4_Add_N3 () { type A = NInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type N7 = NInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > > ; # [allow (non_camel_case_types)] type N4AddN3 = < < A as Add < B > > :: Output as Same < N7 > > :: Output ; assert_eq ! (< N4AddN3 as Integer >:: to_i64 () , < N7 as Integer >:: to_i64 ()) ; }