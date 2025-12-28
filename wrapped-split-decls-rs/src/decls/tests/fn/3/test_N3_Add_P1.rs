use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_N3_Add_P1 () { type A = NInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UTerm , B1 > > ; type N2 = NInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type N3AddP1 = < < A as Add < B > > :: Output as Same < N2 > > :: Output ; assert_eq ! (< N3AddP1 as Integer >:: to_i64 () , < N2 as Integer >:: to_i64 ()) ; }