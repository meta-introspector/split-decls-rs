use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P1_Add_P4 () { type A = PInt < UInt < UTerm , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type P5 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type P1AddP4 = < < A as Add < B > > :: Output as Same < P5 > > :: Output ; assert_eq ! (< P1AddP4 as Integer >:: to_i64 () , < P5 as Integer >:: to_i64 ()) ; }