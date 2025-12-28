use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P3_Add_P5 () { type A = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P8 = PInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B0 > > ; # [allow (non_camel_case_types)] type P3AddP5 = < < A as Add < B > > :: Output as Same < P8 > > :: Output ; assert_eq ! (< P3AddP5 as Integer >:: to_i64 () , < P8 as Integer >:: to_i64 ()) ; }