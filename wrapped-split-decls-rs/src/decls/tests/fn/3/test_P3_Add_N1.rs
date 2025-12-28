use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P3_Add_N1 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = NInt < UInt < UTerm , B1 > > ; type P2 = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; # [allow (non_camel_case_types)] type P3AddN1 = < < A as Add < B > > :: Output as Same < P2 > > :: Output ; assert_eq ! (< P3AddN1 as Integer >:: to_i64 () , < P2 as Integer >:: to_i64 ()) ; }