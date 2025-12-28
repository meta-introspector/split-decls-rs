use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P2_Add_P3 () { type A = PInt < UInt < UInt < UTerm , B1 > , B0 > > ; type B = PInt < UInt < UInt < UTerm , B1 > , B1 > > ; type P5 = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type P2AddP3 = < < A as Add < B > > :: Output as Same < P5 > > :: Output ; assert_eq ! (< P2AddP3 as Integer >:: to_i64 () , < P5 as Integer >:: to_i64 ()) ; }