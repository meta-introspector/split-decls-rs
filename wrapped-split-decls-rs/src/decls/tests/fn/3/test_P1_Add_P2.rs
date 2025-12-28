use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P1_Add_P2 () { type A = PInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type P3 = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P1AddP2 = < < A as Add < B > > :: Output as Same < P3 > > :: Output ; assert_eq ! (< P1AddP2 as Integer >:: to_i64 () , < P3 as Integer >:: to_i64 ()) ; }