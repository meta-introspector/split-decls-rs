use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P3_Add_P4 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P7 = PInt < UInt < UInt < UInt < UTerm , B1 > , B1 > , B1 > > ; # [allow (non_camel_case_types)] type P3AddP4 = < < A as Add < B > > :: Output as Same < P7 > > :: Output ; assert_eq ! (< P3AddP4 as Integer >:: to_i64 () , < P7 as Integer >:: to_i64 ()) ; }