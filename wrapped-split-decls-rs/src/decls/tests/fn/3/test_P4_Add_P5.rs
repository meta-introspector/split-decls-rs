use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [allow (non_snake_case)] fn test_P4_Add_P5 () { type A = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > > ; type B = PInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B1 > > ; type P9 = PInt < UInt < UInt < UInt < UInt < UTerm , B1 > , B0 > , B0 > , B1 > > ; # [allow (non_camel_case_types)] type P4AddP5 = < < A as Add < B > > :: Output as Same < P9 > > :: Output ; assert_eq ! (< P4AddP5 as Integer >:: to_i64 () , < P9 as Integer >:: to_i64 ()) ; }